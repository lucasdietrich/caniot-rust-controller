use std::time::Duration;

use crate::can::{CanInterface, CanStats};
use crate::caniot::{DeviceId, RequestData, ResponseData, EmbeddedFrameWrapper, Response as CaniotResponse, Request as CaniotRequest};
use crate::shared::{Shared, SharedHandle};
use crate::shutdown::{Shutdown, self};
use log::info;
use serde::{Deserialize, Serialize};

use socketcan::CanDataFrame;
use thiserror::Error;
use tokio::select;
use tokio::sync::{mpsc, oneshot};
use tokio::time::sleep;

use super::{ControllerMessage, ControllerHandle};

const CHANNEL_SIZE: usize = 10;

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct CaniotStats {
    pub rx: usize,
    pub tx: usize,
    pub err: usize,
    pub malformed: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CaniotConfig {
}

impl Default for CaniotConfig {
    fn default() -> Self {
        CaniotConfig {
            
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Device {
    device_id: DeviceId,
}

pub struct Controller {
    pub iface: CanInterface,
    pub stats: CaniotStats,
    
    devices: [Device; 63],
    
    shutdown: Shutdown,
    receiver: mpsc::Receiver<ControllerMessage>,
    handle: ControllerHandle,
}

impl Controller {
    pub(crate) fn new(iface: CanInterface, shutdown: Shutdown) -> Self {
        let (sender, receiver) = mpsc::channel(CHANNEL_SIZE);

        Self {
            iface,
            stats: CaniotStats::default(),
            devices: [Device { device_id: DeviceId { class: 0, sub_id: 0 } }; 63],
            shutdown,
            receiver,
            handle: ControllerHandle { sender },
        }
    }

    pub fn get_handle(&self) -> ControllerHandle {
        self.handle.clone()
    }

    fn handle_message(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::GetStats { respond_to } => {
                let _ = respond_to.send((self.stats, self.iface.stats));
            },
            ControllerMessage::Query { respond_to } => {
                let _ = respond_to.send(());
            }
        }
    }

    fn handle_can_frame(&mut self, frame: CanDataFrame) {
        let frame: Result<CaniotResponse, _> = EmbeddedFrameWrapper(frame).try_into();
        match frame {
            Ok(frame) => {
                self.stats.rx += 1;
                info!("RX {}", frame);
            }
            Err(err) => {
                self.stats.malformed += 1;
                error!("Failed to convert into CANIOT frame {}", err)
            }
        }
    }

    pub async fn run(mut self) -> Result<(), ()> {
        loop {
            select! {
                Some(message) = self.receiver.recv() => {
                    self.handle_message(message);
                },
                Some(frame) = self.iface.recv_poll() => {
                    self.handle_can_frame(frame);
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal");
                    break;
                }
            }
        }

        Ok(())
    }

    // pub async fn query(&self, _request: Request) -> Result<Response, Error> {
    //     Err(Error::Timeout)
    // }
}

pub struct Query {
    did: DeviceId,
    timeout: f32,
}

pub struct Response {
    did: DeviceId,
    data: ResponseData,
    duration: f32,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Timeout")]
    Timeout,
}