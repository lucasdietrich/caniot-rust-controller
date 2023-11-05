use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use crate::can::{CanInterface, CanInterfaceError, CanStats};
use crate::caniot::{
    DeviceId, EmbeddedFrameWrapper, ProtocolError as CaniotProtocolError, Request as CaniotRequest,
    RequestData, Response as CaniotResponse, ResponseData,
    is_response_to, ConversionError
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

    #[error("CAN Interface Error: {0}")]
    CanError(#[from] CanInterfaceError),

    #[error("CANIOT Error: {0}")]
    CaniotProtocolError(#[from] CaniotProtocolError),

    #[error("Conversion Error: {0}")]
    ConversionError(#[from] ConversionError),
}

#[derive(Clone, Copy, Debug)]
struct Device {
    device_id: DeviceId,
}

type PendingQueryClosure = Pin<Box<dyn FnOnce(Result<CaniotResponse, ControllerError>) -> () + Send + Sync + 'static>>;

struct PendingQuery {
    // query pending
    query: CaniotRequest,

    // closure to call when response is received
    closure: PendingQueryClosure,

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

    pub async fn query(
        &mut self,
        request: CaniotRequest,
        timeout_ms: u32,
        closure: PendingQueryClosure,
    ) -> Result<(), ControllerError> {
        let can_frame = request.to_can_frame()?;
        println!("Sending CAN frame: {:?} -> {:?}", request, can_frame);
        self.iface.send(can_frame).await?;

        let pending_query = PendingQuery {
            query: request,
            closure,
            timeout_ms,
            sent_at: std::time::Instant::now(),
        };

        self.pending_queries.push(pending_query);

        Ok(())
    }

    async fn handle_can_frame(&mut self, frame: CanDataFrame) -> Result<(), ControllerError> {
        let frame: CaniotResponse = EmbeddedFrameWrapper(frame).try_into()?;
        
        info!("RX {}", frame);

        // update stats
        self.stats.rx += 1;

        let pq = self.pending_queries
            .iter()
            .position(|pq| is_response_to(&pq.query, &frame).is_response())
            .map(|idx| self.pending_queries.remove(idx));

        if let Some(pq) = pq {
            let closure = pq.closure;
            closure(Ok(frame));
        } else {
            error!("Received response to unknown query: {}", frame);
        }

        Ok(())
    }

    pub async fn run(mut self) -> Result<(), ()> {
        loop {
            select! {
                Some(message) = self.receiver.recv() => {
                    handle_message(&mut self, message).await;
                },
                Some(frame) = self.iface.recv_poll() => {
                    match self.handle_can_frame(frame).await {
                        Ok(_) => {
                        },
                        Err(ControllerError::ConversionError(err)) => {
                            self.stats.malformed += 1;
                            error!("Failed to convert into CANIOT frame {}", err)
                        },
                        _ => {}
                    }
                },
                // TODO handle pending queries timeout
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal");
                    break;
                }
            }
        }

        Ok(())
    }
}
