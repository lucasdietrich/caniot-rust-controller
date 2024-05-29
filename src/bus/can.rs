use futures_util::{SinkExt, StreamExt};
use log::error;

use crate::caniot::{CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK};
use socketcan::tokio::CanSocket;
use socketcan::{CanDataFrame, CanFilter, CanFrame, Socket, SocketOptions};

use super::{CanConfig, CanInterfaceError, CanInterfaceTrait, CanStats};

pub struct CanInterface {
    sock: CanSocket,
    pub stats: CanStats,
}

#[async_trait]
impl CanInterfaceTrait for CanInterface {
    async fn new(config: &CanConfig) -> Result<Self, CanInterfaceError> {
        let sock = CanSocket::open(&config.interface)?;
        let filter = CanFilter::new(CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK);
        sock.set_filters(&[filter])?;
        Ok(Self {
            sock,
            stats: CanStats::default(),
        })
    }

    async fn send(&mut self, frame: CanDataFrame) -> Result<(), CanInterfaceError> {
        self.sock.send(frame.into()).await?;
        self.stats.tx += 1;
        Ok(())
    }

    async fn recv_poll(&mut self) -> Option<CanDataFrame> {
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

    fn get_stats(&self) -> CanStats {
        self.stats
    }
}
