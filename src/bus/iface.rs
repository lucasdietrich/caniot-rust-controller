use log::error;
use serde::{Deserialize, Serialize};

use socketcan::{CanDataFrame, Error as CanError};
use thiserror::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CanConfig {
    pub interface: String,
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

#[async_trait]
pub trait CanInterfaceTrait
where
    Self: Sized,
{
    async fn new(config: &CanConfig) -> Result<Self, CanInterfaceError>;

    async fn send(&mut self, frame: CanDataFrame) -> Result<(), CanInterfaceError>;

    async fn recv_poll(&mut self) -> Option<CanDataFrame>;

    fn get_stats(&self) -> CanStats;
}
