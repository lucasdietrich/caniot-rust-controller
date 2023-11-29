use std::sync::Arc;
use std::time::{Duration, Instant};

use itertools::{partition, Itertools};
use tokio::runtime::Runtime;

use crate::can::{CanInterface, CanInterfaceError};
use crate::caniot;
use crate::caniot::DeviceId;
use crate::shutdown::Shutdown;

use super::{actor, ManagedDeviceTrait, ManagedDeviceError, DeviceTrait};
use super::device::{Device, DeviceStats};
use super::traits::ControllerAPI;

use log::info;
use serde::{Deserialize, Serialize};

use socketcan::CanDataFrame;
use thiserror::Error;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

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
}

#[derive(Debug)]
struct PendingQuery {
    // query pending
    query: caniot::Request,

    // closure to call when response is received
    sender: oneshot::Sender<Result<caniot::Response, ControllerError>>,

    // timeout in milliseconds
    timeout_ms: u32,

    // time when query was sent
    sent_at: std::time::Instant,
}

pub struct Controller {
    pub iface: CanInterface,
    pub stats: ControllerStats,
    pub config: CaniotConfig,

    // managed_devices: Vec<Box<dyn ManagedDeviceTrait<Error = ManagedDeviceError>>>,
    devices: Vec<Box<dyn DeviceTrait<Error = ManagedDeviceError>>>,
    pending_queries: Vec<PendingQuery>,

    rt: Arc<Runtime>,
    shutdown: Shutdown,

    receiver: mpsc::Receiver<actor::ControllerMessage>,
    handle: actor::ControllerHandle,
}

impl Controller {
    pub(crate) fn new(
        iface: CanInterface,
        config: CaniotConfig,
        // managed_devices: Vec<Box<dyn ManagedDeviceTrait<Error = ManagedDeviceError>>>,
        managed_devices: Vec<Box<dyn DeviceTrait<Error = ManagedDeviceError>>>,
        shutdown: Shutdown,
        rt: Arc<Runtime>,
    ) -> Result<Self, ControllerError> {

        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);

        // sanity check on managed devices
        if managed_devices.len() > DEVICES_COUNT {
            return Err(ControllerError::DuplicateDID);
        }


        // // filter duplicates
        // let managed_devices: Vec<Box<_>> = managed_devices
        //     .into_iter()
        //     .sorted_by_key(|device| device.get_did().as_u8())
        //     .dedup_by(|a, b| a.get_did().as_u8() == b.get_did().as_u8())
        //     .collect();
        

        // // initialize devices
        // let devices = (0..DEVICES_COUNT)
        //     .into_iter()
        //     .map(|did| Device {
        //         device_id: DeviceId::new(did as u8).unwrap(),
        //         last_seen: None,
        //         stats: DeviceStats::default(),
        //     })
        //     .collect::<Vec<_>>()
        //     .try_into()
        //     .unwrap();

        Ok(Self {
            iface,
            stats: ControllerStats::default(),
            config,
            devices: managed_devices,
            pending_queries: Vec::new(),
            rt,
            shutdown,
            receiver,
            handle: actor::ControllerHandle { sender },
        })
    }

    pub fn get_handle(&self) -> actor::ControllerHandle {
        self.handle.clone()
    }

    // pub fn get_device_handle(&self, did: DeviceId) -> DeviceHandle {
    //     self.handle.get_device(did)
    // }

    async fn send_caniot_frame(
        &mut self,
        request: &caniot::Request,
    ) -> Result<(), ControllerError> {
        info!("TX {}", request);
        let can_frame = request.to_can_frame()?;
        self.iface.send(can_frame).await?;
        self.stats.tx += 1;
        Ok(())
    }

    pub async fn query_sched(
        &mut self,
        request: caniot::Request,
        timeout_ms: u32,
        sender: oneshot::Sender<Result<caniot::Response, ControllerError>>,
    ) {
        if request.device_id == DeviceId::BROADCAST {
            error!("BROADCAST query not supported");
            let _ = sender.send(Err(ControllerError::UnsupportedQuery));
        } else if let Err(err) = self.send_caniot_frame(&request).await {
            error!("Failed to send CANIOT frame: {:?}", err);
            let _ = sender.send(Err(err)); // Send None, but do not panic if receiver is dropped
        } else {
            self.pending_queries.push(PendingQuery {
                query: request,
                sender,
                timeout_ms,
                sent_at: std::time::Instant::now(),
            })
        }
    }

    async fn handle_can_frame(&mut self, frame: CanDataFrame) -> Result<(), ControllerError> {
        let frame: caniot::Response = caniot::EmbeddedFrameWrapper(frame).try_into()?;

        info!("RX {}", frame);

        // update stats
        self.stats.rx += 1;

        // let device_index = frame.device_id.as_u8() as usize;
        // let device = &mut self.devices[device_index];

        // device.process_incoming_response(&frame);

        // TODO broadcast should be handled differently as the oneshot channel cannot be used to send multiple responses

        let pivot = self
            .pending_queries
            .partition_point(|pq| caniot::is_response_to(&pq.query, &frame).is_response());

        // if a frame can answer multiple pending queries, remove all of them
        for pq in self.pending_queries.drain(..pivot) {
            let _ = pq.sender.send(Ok(frame.clone())); // Do not panic if receiver is dropped
        }

        Ok(())
    }

    fn time_to_next_timeout(&self) -> Duration {
        // TODO improve this code
        let now = std::time::Instant::now();

        let neareast_timeout = self
            .pending_queries
            .iter()
            .map(|pq| pq.sent_at + Duration::from_millis(pq.timeout_ms as u64))
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

    async fn handle_pending_queries_timeout(&mut self) {
        let now = std::time::Instant::now();

        // place all timed out queries at the end of the vector
        let split_index = partition(&mut self.pending_queries, |pq| {
            now.duration_since(pq.sent_at) < Duration::from_millis(pq.timeout_ms as u64)
        });

        // remove timed out queries
        let timed_out_queries = self.pending_queries.split_off(split_index);

        // send timeout to all timed out queries
        for pq in timed_out_queries {
            warn!(
                "Pending query timeout {} after {} ms",
                pq.query, pq.timeout_ms
            );
            let _ = pq.sender.send(Err(ControllerError::Timeout)); // Do not panic if receiver is dropped
        }
    }

    pub async fn run(mut self) -> Result<(), ()> {
        loop {
            let time_to_next_timeout = self.time_to_next_timeout();

            select! {
                Some(message) = self.receiver.recv() => {
                    actor::handle_message(&mut self, message).await;
                },
                Some(frame) = self.iface.recv_poll() => {
                    match self.handle_can_frame(frame).await {
                        Ok(_) => {
                            // nothing more to do
                        },
                        Err(ControllerError::CaniotConversionError(err)) => {
                            self.stats.malformed += 1;
                            error!("Failed to convert into CANIOT frame {}", err)
                        },
                        _ => {}
                    }
                },
                _ = sleep(time_to_next_timeout) => {
                    // timeout handled in handle_pending_queries_timeout()
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal");
                    break;
                }
            }

            self.handle_pending_queries_timeout().await;
        }

        Ok(())
    }

    pub fn get_devices_stats(&self) -> Vec<actor::DeviceStatsEntry> {
        vec![]
        // self.devices
        //     .iter()
        //     .filter(|device| device.last_seen.is_some())
        //     .sorted_by_key(|device| device.last_seen.unwrap())
        //     .rev()
        //     .map(|device| actor::DeviceStatsEntry {
        //         device_id_did: device.device_id.as_u8(),
        //         device_id: device.device_id,
        //         stats: device.stats,
        //     })
        //     .collect()
    }
}

#[async_trait]
impl ControllerAPI for Controller {
    async fn query(
        &mut self,
        frame: caniot::Request,
        timeout_ms: u32,
    ) -> Result<caniot::Response, ControllerError> {
        let (sender, receiver) = oneshot::channel();
        self.query_sched(frame, timeout_ms, sender).await;
        self.rt
            .spawn(async move { receiver.await.unwrap() })
            .await
            .unwrap()
    }

    async fn send(&mut self, frame: caniot::Request) -> Result<(), ControllerError> {
        self.send_caniot_frame(&frame).await
    }
}
