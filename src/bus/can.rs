
use log::{error};
use serde::{Deserialize, Serialize};

use socketcan::{Error as CanError};
use thiserror::Error;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CanConfig {
    interface: String,
}

impl Default for CanConfig {
    fn default() -> Self {
        CanConfig {
            interface: "can0".to_string(),
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct CanStats {
    pub rx: usize,
    pub tx: usize,
    pub err: usize,
    pub unhandled: usize,
}

#[derive(Error, Debug)]
pub enum CanInterfaceError {
    #[error("SocketCAN error: {0}")]
    CanError(#[from] CanError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(not(feature = "emu"))]
pub struct CanInterface {
    sock: CanSocket,
    pub stats: CanStats,
}

#[cfg(not(feature = "emu"))]
impl CanInterface {
    pub async fn new(config: &CanConfig) -> Result<Self, CanInterfaceError> {
        let sock = CanSocket::open(&config.interface)?;
        let filter = CanFilter::new(CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK);
        sock.set_filters(&[filter])?;
        Ok(Self {
            sock,
            stats: CanStats::default(),
        })
    }

    pub async fn send(&mut self, frame: CanFrame) -> Result<(), CanInterfaceError> {
        self.sock.send(frame).await?;
        self.stats.tx += 1;
        Ok(())
    }

    pub async fn recv_poll(&mut self) -> Option<CanDataFrame> {
        if let Some(result) = self.sock.next().await {
            match result {
                Ok(CanFrame::Data(frame)) => {
                    self.stats.rx += 1;
                    return Some(frame);
                }
                Ok(CanFrame::Remote(frame)) => {
                    warn!("Unhandled {:?}", frame);
                    self.stats.unhandled += 1;
                }
                Ok(CanFrame::Error(frame)) => {
                    error!("{:?}", frame);
                    self.stats.unhandled += 1;
                }
                Err(err) => {
                    error!("{}", err);
                    self.stats.err += 1;
                }
            };
        };
        None
    }

    pub fn get_stats(&self) -> CanStats {
        self.stats
    }
}