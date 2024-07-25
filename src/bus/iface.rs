use log::error;
use serde::{Deserialize, Serialize};

use socketcan::{CanDataFrame, Error as CanError};
use thiserror::Error;

use crate::utils::{PrometheusExporterTrait, PrometheusNoLabel};

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

impl PrometheusExporterTrait for CanStats {
    type Label = PrometheusNoLabel;
    fn export(&self, _labels: impl AsRef<[Self::Label]>) -> String {
        format!(
            "can_rx {}\n\
            can_tx {}\n\
            can_err {}\n\
            can_unhandled {}\n",
            self.rx, self.tx, self.err, self.unhandled
        )
    }
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

    // Allow to perform alternatives operations on the interface (e.g. send emulated events, change filters, inhibit, etc.)
    fn ioctl(&mut self, _cmd: u32, _arg: u32) -> Result<(), CanInterfaceError> {
        error!("ioctl not implemented");
        Ok(())
    }
}

pub const CAN_IOCTL_SEND_EMU_EVENT: u32 = 0xFFFF_0001;
