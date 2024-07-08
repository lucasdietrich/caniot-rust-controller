use std::{pin::Pin, task::Poll};

use futures::future::Pending;
use socketcan::CanDataFrame;
use tokio::sync::{
    mpsc::{self, error::TrySendError},
    TryAcquireError,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CanTunnelError {
    #[error("Tunnel is already established")]
    AlreadyEstablished,
}

struct Tunnel {
    rx_queue: mpsc::Sender<CanDataFrame>,
    tx_queue: mpsc::Receiver<CanDataFrame>,
}

#[derive(Default)]
pub struct CanTunnelContextServer {
    tunnel: Option<Tunnel>,
}

impl CanTunnelContextServer {
    pub fn establish_can_tunnel(
        &mut self,
        rx_queue: mpsc::Sender<CanDataFrame>,
        tx_queue: mpsc::Receiver<CanDataFrame>,
    ) -> Result<(), CanTunnelError> {
        if self.tunnel.is_some() {
            return Err(CanTunnelError::AlreadyEstablished);
        }

        self.tunnel = Some(Tunnel { rx_queue, tx_queue });
        Ok(())
    }

    pub fn close_tunnel(&mut self) {
        self.tunnel = None;
    }

    pub fn notify_rx(&mut self, frame: CanDataFrame) {
        if let Some(tunnel) = &self.tunnel {
            match tunnel.rx_queue.try_send(frame) {
                Ok(_) => {}
                Err(TrySendError::Closed(frame)) => {
                    log::error!("CanTunnel rx_queue closed, closing tunnel");
                    self.close_tunnel();
                }
                Err(TrySendError::Full(frame)) => {
                    log::error!("CanTunnel rx_queue full, dropping frame");
                }
            }
        }
    }

    // Return futures
    pub async fn poll_tx(&mut self) -> Option<CanDataFrame> {
        if let Some(tunnel) = &mut self.tunnel {
            if let Some(frame) = tunnel.tx_queue.recv().await {
                Some(frame)
            } else {
                // tx_queue closed
                self.close_tunnel();
                None
            }
        } else {
            futures::future::pending().await
        }
    }
}
