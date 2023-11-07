use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use itertools::partition;

use crate::can::{CanInterface, CanInterfaceError, CanStats};
use crate::caniot::{
    is_response_to, ConversionError as CaniotConversionError, DeviceId, EmbeddedFrameWrapper,
    ProtocolError as CaniotProtocolError, Request as CaniotRequest, RequestData,
    Response as CaniotResponse, ResponseData,
};
use crate::controller;
use crate::shared::{Shared, SharedHandle};
use crate::shutdown::{self, Shutdown};
use log::info;
use serde::{Deserialize, Serialize};

use socketcan::CanDataFrame;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

use super::actor::{handle_message, ControllerHandle, ControllerMessage};

const CHANNEL_SIZE: usize = 10;
const DEVICES_COUNT: usize = 63;

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct CaniotStats {
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

    #[error("CAN Interface Error: {0}")]
    CanError(#[from] CanInterfaceError),

    #[error("CANIOT Error: {0}")]
    CaniotProtocolError(#[from] CaniotProtocolError),

    #[error("Conversion Error: {0}")]
    CaniotConversionError(#[from] CaniotConversionError),
}

#[derive(Clone, Copy, Debug)]
struct Device {
    device_id: DeviceId,
}

#[derive(Debug)]
struct PendingQuery {
    // query pending
    query: CaniotRequest,

    // closure to call when response is received
    sender: oneshot::Sender<Result<CaniotResponse, ControllerError>>,

    // timeout in milliseconds
    timeout_ms: u32,

    // time when query was sent
    sent_at: std::time::Instant,
}

pub struct Controller {
    pub iface: CanInterface,
    pub stats: CaniotStats,

    devices: [Device; DEVICES_COUNT],
    pending_queries: Vec<PendingQuery>,

    rt: Arc<Runtime>,
    shutdown: Shutdown,

    receiver: mpsc::Receiver<ControllerMessage>,
    pub handle: ControllerHandle,
}

impl Controller {
    pub(crate) fn new(iface: CanInterface, rt: Arc<Runtime>, shutdown: Shutdown) -> Self {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);

        Self {
            iface,
            stats: CaniotStats::default(),
            devices: [Device {
                device_id: DeviceId {
                    class: 0,
                    sub_id: 0,
                },
            }; 63],
            pending_queries: Vec::new(),
            rt,
            shutdown,
            receiver,
            handle: ControllerHandle { sender },
        }
    }

    pub fn get_handle(&self) -> ControllerHandle {
        self.handle.clone()
    }

    async fn send_caniot_frame(&mut self, request: &CaniotRequest) -> Result<(), ControllerError> {
        info!("TX {}", request);
        let can_frame = request.to_can_frame()?;
        self.iface.send(can_frame).await?;
        self.stats.tx += 1;
        Ok(())
    }

    pub async fn query(
        &mut self,
        request: CaniotRequest,
        timeout_ms: u32,
        sender: oneshot::Sender<Result<CaniotResponse, ControllerError>>,
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
        let frame: CaniotResponse = EmbeddedFrameWrapper(frame).try_into()?;

        info!("RX {}", frame);

        // update stats
        self.stats.rx += 1;

        // TODO if a frame can answer multiple pending queries, remove all of them
        // TODO broadcast should be handled differently as the oneshot channel cannot be used to send multiple responses

        let pq = self
            .pending_queries
            .iter()
            .position(|pq| is_response_to(&pq.query, &frame).is_response())
            .map(|idx| self.pending_queries.remove(idx));

        if let Some(pq) = pq {
            let _ = pq.sender.send(Ok(frame)); // Do not panic if receiver is dropped
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

    async fn handle_pending_queries(&mut self) {
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
                    handle_message(&mut self, message).await;
                },
                Some(frame) = self.iface.recv_poll() => {
                    match self.handle_can_frame(frame).await {
                        Ok(_) => {
                        },
                        Err(ControllerError::CaniotConversionError(err)) => {
                            self.stats.malformed += 1;
                            error!("Failed to convert into CANIOT frame {}", err)
                        },
                        _ => {}
                    }
                },
                _ = sleep(time_to_next_timeout) => {
                    // timeout handled in handle_pending_queries()
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal");
                    break;
                }
            }

            self.handle_pending_queries().await;
        }

        Ok(())
    }
}
