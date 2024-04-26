use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use embedded_can::Frame;
use itertools::{partition, Itertools};
use tokio::runtime::Runtime;

use crate::bus::{CanInterface, CanInterfaceError};
use crate::caniot::emu::{
    emu_pool1_add_devices_to_iface, emu_pool2_realistic_add_devices_to_iface,
};
use crate::caniot::{self, emu};
use crate::caniot::{DeviceId, Request};
use crate::controller::{
    Device, DeviceActionTrait, DeviceError, DeviceEvent, DeviceProcessOutput, DeviceTrait,
};
use crate::shutdown::Shutdown;

use super::super::ControllerAPI;
use super::super::{actor, DemoController, GarageController};
use super::attach::device_attach_controller;
use super::PendingQuery;

use log::info;
use serde::{Deserialize, Serialize};

use socketcan::CanFrame;
use thiserror::Error;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

const QUERY_DEFAULT_TIMEOUT_MS: u32 = 1000; // 1s
const CHANNEL_SIZE: usize = 10;
const DEVICES_COUNT: usize = 63;

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct ControllerStats {
    pub rx: usize,
    pub tx: usize,
    pub err: usize,
    pub malformed: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaniotConfig {}

impl Default for CaniotConfig {
    fn default() -> Self {
        CaniotConfig {}
    }
}

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Timeout Error")]
    Timeout,

    #[error("Unsupported query Error")]
    UnsupportedQuery,

    #[error("Duplicate device Error")]
    DuplicateDID,

    #[error("CAN Interface Error: {0}")]
    CanError(#[from] CanInterfaceError),

    #[error("CANIOT Error: {0}")]
    CaniotProtocolError(#[from] caniot::ProtocolError),

    #[error("Conversion Error: {0}")]
    CaniotConversionError(#[from] caniot::ConversionError),

    #[error("Device error: {0}")]
    DeviceError(#[from] DeviceError),
}

pub struct Controller {
    // CAN interface
    pub iface: CanInterface,

    // Service
    pub config: CaniotConfig,
    pub stats: ControllerStats,
    rt: Arc<Runtime>,
    shutdown: Shutdown,
    receiver: mpsc::Receiver<actor::ControllerMessage>,
    handle: actor::ControllerHandle,

    // State
    pending_queries: Vec<PendingQuery>,
    devices: HashMap<DeviceId, Device>,
}

impl Controller {
    pub(crate) fn new(
        mut iface: CanInterface,
        config: CaniotConfig,
        shutdown: Shutdown,
        rt: Arc<Runtime>,
    ) -> Result<Self, ControllerError> {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);

        #[cfg(feature = "emu")]
        {
            // emu_pool1_add_devices_to_iface(&mut iface);
            emu_pool2_realistic_add_devices_to_iface(&mut iface);
        }

        Ok(Self {
            iface,
            config,
            stats: ControllerStats::default(),
            rt,
            shutdown,
            receiver,
            handle: actor::ControllerHandle::new(sender),
            pending_queries: Vec::new(),
            devices: HashMap::new(),
        })
    }

    pub fn get_handle(&self) -> actor::ControllerHandle {
        self.handle.clone()
    }

    pub async fn send_caniot_frame(
        &mut self,
        request: &caniot::Request,
    ) -> Result<(), ControllerError> {
        info!("TX {}", request);

        let can_frame = request.into();

        self.iface.send(can_frame).await?;
        self.stats.tx += 1;
        Ok(())
    }

    pub async fn query_sched(
        &mut self,
        request: caniot::Request,
        timeout_ms: Option<u32>,
        sender: oneshot::Sender<Result<caniot::Response, ControllerError>>,
    ) {
        let timeout_ms = timeout_ms.unwrap_or(QUERY_DEFAULT_TIMEOUT_MS);

        if request.device_id == DeviceId::BROADCAST {
            error!("BROADCAST query not supported");
            let _ = sender.send(Err(ControllerError::UnsupportedQuery));
        } else if let Err(err) = self.send_caniot_frame(&request).await {
            error!("Failed to send CANIOT frame: {:?}", err);
            let _ = sender.send(Err(err)); // Send None, but do not panic if receiver is dropped
        } else {
            self.pending_queries
                .push(PendingQuery::new(request, sender, timeout_ms));
        }
    }

    async fn handle_caniot_response(
        &mut self,
        frame: caniot::Response,
    ) -> Result<(), ControllerError> {
        self.stats.rx += 1;

        // Get or create device
        let device = if let Some(device) = self.devices.get_mut(&frame.device_id) {
            device
        } else {
            let mut new_device = Device::new(frame.device_id);
            device_attach_controller(&mut new_device);
            self.devices.insert(frame.device_id, new_device);
            self.devices.get_mut(&frame.device_id).unwrap()
        };

        // Update device stats
        let device_result = device.handle_frame(&frame.data)?;

        // Set next process time
        device.schedule_next_process_in(device_result.next_process);

        // TODO broadcast should be handled differently as the oneshot channel cannot be used to send multiple responses
        let pivot = self
            .pending_queries
            .partition_point(|pq| pq.match_response(&frame));

        // if a frame can answer multiple pending queries, remove all of them
        for pq in self.pending_queries.drain(..pivot) {
            pq.reply_with_frame(frame.clone());
        }

        // Send requests to device
        let device_did = device.did;
        for request_data in device_result.requests {
            let request = Request::new(device_did, request_data);
            self.send_caniot_frame(&request).await?;
        }

        Ok(())
    }

    fn time_to_next_pq_timeout(&self) -> Duration {
        // TODO improve this code
        let now = std::time::Instant::now();

        let neareast_timeout = self
            .pending_queries
            .iter()
            .map(|pq| pq.get_timeout_instant())
            .min();

        if let Some(neareast_timeout) = neareast_timeout {
            if neareast_timeout > now {
                neareast_timeout - now
            } else {
                Duration::from_millis(0)
            }
        } else {
            Duration::MAX
        }
    }

    fn time_to_next_device_process(&self) -> Duration {
        let now = std::time::Instant::now();

        let nearest_process = self
            .devices
            .iter()
            .filter_map(|(_did, device)| {
                if let Some(next_process) = device.next_process_time() {
                    Some(next_process)
                } else {
                    None
                }
            })
            .min();

        if let Some(nearest_process) = nearest_process {
            if nearest_process > now {
                nearest_process - now
            } else {
                Duration::from_millis(0)
            }
        } else {
            Duration::MAX
        }
    }

    async fn handle_pending_queries_timeout(&mut self) {
        let now = std::time::Instant::now();

        // place all timed out queries at the end of the vector
        let split_index = partition(&mut self.pending_queries, |pq| !pq.has_timed_out(&now));

        // remove timed out queries
        let timed_out_queries = self.pending_queries.split_off(split_index);

        // send timeout to all timed out queries
        for pq in timed_out_queries {
            warn!(
                "Pending query {} timed out after {} ms",
                pq.query, pq.timeout_ms
            );
            pq.reply(Err(ControllerError::Timeout));
        }
    }

    async fn process_devices(&mut self) -> Result<(), ControllerError> {
        let mut results = Vec::new();

        // Process devices
        for (did, device) in self.devices.iter_mut() {
            if device.needs_process() {
                let device_result = device.handle_event(&DeviceEvent::Process)?;
                device.mark_processed();
                device.schedule_next_process_in(device_result.next_process);
                results.push((did.to_owned(), device_result));
            }
        }

        // Send requests to device
        for (did, result) in results {
            for request_data in result.requests {
                let request = Request::new(did, request_data);
                self.send_caniot_frame(&request).await?;
            }
        }

        Ok(())
    }

    pub async fn run(mut self) -> Result<(), ()> {
        loop {
            let time_to_next_pq_timeout = self.time_to_next_pq_timeout();
            let time_to_next_device_process = self.time_to_next_device_process();
            let sleep_time = time_to_next_pq_timeout.min(time_to_next_device_process);

            select! {
                Some(message) = self.receiver.recv() => {
                    self.handle_api_message(message).await;
                },
                Some(frame) = self.iface.recv_poll() => {
                    match caniot::Response::try_from(frame) {
                        Ok(frame) => {
                            info!("RX {}", frame);
                            let result = self.handle_caniot_response(frame).await;
                            if let Err(err) = result {
                                error!("Failed to handle CANIOT frame {}", err);
                            }
                        },
                        Err(err) => {
                            self.stats.malformed += 1;
                            error!("Failed to convert into CANIOT frame {}", err)
                        },
                    }
                },
                _ = sleep(sleep_time) => {
                    // Timeout of pending queries handled in handle_pending_queries_timeout()
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal, exiting ...");
                    break;
                }
            }

            self.handle_pending_queries_timeout().await;
            self.process_devices().await.unwrap();
        }

        Ok(())
    }

    pub fn get_devices_stats(&self) -> Vec<actor::DeviceStatsEntry> {
        self.devices
            .iter()
            .filter(|(_did, device)| device.last_seen.is_some())
            .sorted_by_key(|(_did, device)| device.last_seen.unwrap())
            .rev()
            .map(|(_did, device)| actor::DeviceStatsEntry {
                did: device.did,
                last_seen: device.last_seen,
                stats: device.stats,
            })
            .collect()
    }
}
