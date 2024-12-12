use std::{sync::Arc, time::Instant};

use chrono::Utc;
use thiserror::Error;
use tokio::{select, sync::mpsc, time::sleep};

use crate::{
    bus::CanInterfaceTrait,
    controller::{
        caniot_controller::caniot_devices_controller::{
            CaniotControllerError, CaniotDevicesController,
        },
        copro_controller::CoproController,
        handle::{self, ControllerMessage},
        CaniotConfig,
    },
    coprocessor::CoproHandle,
    database::Storage,
    shutdown::Shutdown,
};

use super::{ControllerCoreStats, ControllerStats};

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Caniot controller error: {0}")]
    CaniotError(#[from] CaniotControllerError),

    #[cfg(feature = "can-tunnel")]
    #[error("Can tunnel error: {0}")]
    CanTunnelError(#[from] super::can_tunnel::CanTunnelError),

    #[cfg(feature = "ble-copro")]
    #[error("BLE copro error: {0}")]
    BleCoproError(#[from] crate::controller::copro_controller::CoproError),
}

pub struct Controller<IF: CanInterfaceTrait> {
    caniot: CaniotDevicesController<IF>,
    copro: CoproController,

    shutdown: Shutdown,

    receiver: mpsc::Receiver<handle::ControllerMessage>,
    handle: handle::ControllerHandle,

    stats: ControllerCoreStats,
}

const API_CHANNEL_SIZE: u32 = 10;

impl<IF: CanInterfaceTrait> Controller<IF> {
    pub(crate) fn new(
        iface: IF,
        caniot_config: CaniotConfig,
        copro_handle: CoproHandle,
        storage: Arc<Storage>,
        shutdown: Shutdown,
    ) -> Result<Self, ControllerError> {
        let (sender, receiver) = mpsc::channel(
            caniot_config
                .inernal_api_mpsc_size
                .unwrap_or(API_CHANNEL_SIZE) as usize,
        );

        Ok(Self {
            caniot: CaniotDevicesController::new(iface, caniot_config, storage)?,
            handle: handle::ControllerHandle::new(sender),
            copro: CoproController::new(copro_handle)?,
            receiver,
            shutdown,
            stats: ControllerCoreStats::default(),
        })
    }

    pub fn get_handle(&self) -> handle::ControllerHandle {
        self.handle.clone()
    }

    pub async fn run(mut self) -> Result<(), ()> {
        let _ = self.caniot.start().await;

        loop {
            let sys_now = Instant::now();
            let utc_now = Utc::now();

            let sleep_time = self.caniot.loop_process(&sys_now, &utc_now).await;

            let tunnel_poll_rx = self.caniot.tunnel_poll_tx();

            select! {
                Some(message) = self.receiver.recv() => {
                    let _ = self.handle_api_message(message).await;
                },
                Some(frame) = self.caniot.iface.recv_poll() => {
                    self.caniot.handle_can_frame(frame).await;
                },
                Some(copro_message) = self.copro.poll_message() => {
                    self.copro.handle_message(copro_message).await;
                },
                Some(frame) = tunnel_poll_rx => {
                    // If frame is received from tunnel, send it to the bus
                    #[cfg(feature = "can-tunnel")]
                    let _ = self.caniot.iface.send(frame).await;
                },
                _ = sleep(sleep_time) => {
                    // Timeout of pending queries handled in handle_pending_queries_timeout()
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal, exiting ...");
                    break;
                }
            }
            self.stats.loop_runs += 1;
        }

        Ok(())
    }

    pub async fn handle_api_message(
        &mut self,
        message: ControllerMessage,
    ) -> Result<(), ControllerError> {
        self.stats.api_rx += 1;
        match message {
            ControllerMessage::GetStats { respond_to } => {
                let stats = ControllerStats {
                    caniot: self.caniot.stats,
                    core: self.stats,
                    can: self.caniot.iface.get_stats(),
                };
                let _ = respond_to.send(stats);
            }
            ControllerMessage::CaniotMessage(caniot_message) => {
                self.caniot.handle_api_message(caniot_message).await?;
            }
            ControllerMessage::CoprocessorMessage(copro_message) => {
                self.copro.handle_api_message(copro_message).await?;
            }
        }

        Ok(())
    }
}
