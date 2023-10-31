use std::time::Duration;

use crate::can::CanInterface;
use crate::caniot::{DeviceId, RequestData, ResponseData, EmbeddedFrameWrapper, Response as CaniotResponse, Request as CaniotRequest};
use crate::shared::{Shared, SharedHandle};
use crate::shutdown::{Shutdown, self};
use log::info;
use serde::{Deserialize, Serialize};

use socketcan::CanDataFrame;
use thiserror::Error;
use tokio::select;
use tokio::time::sleep;

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
    shutdown: Shutdown,
    pub stats: CaniotStats,

    devices: [Device; 63],
}

impl Controller {
    pub(crate) fn new(iface: CanInterface, shutdown: Shutdown) -> Self {
        Self {
            iface,
            shutdown,
            stats: CaniotStats::default(),
            devices: [Device { device_id: DeviceId { class: 0, sub_id: 0 } }; 63],
        }
    }

    fn handle_can_data_frame(&mut self, frame: CanDataFrame) {
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
                Some(frame) = self.iface.recv_poll() => {
                    self.handle_can_data_frame(frame);
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