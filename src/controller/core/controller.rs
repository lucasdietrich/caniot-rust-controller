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
        handle::{self, ControllerApiMessage},
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

    #[cfg(feature = "ble-copro")]
    #[error("BLE copro error: {0}")]
    BleCoproError(#[from] crate::controller::copro_controller::CoproError),
}

pub struct Controller<IF: CanInterfaceTrait> {
    caniot: CaniotDevicesController<IF>,
    copro: CoproController,

    shutdown: Shutdown,

    api_receiver: mpsc::Receiver<handle::ControllerApiMessage>,
    api_handle: handle::ControllerHandle,

    stats: ControllerCoreStats,
}

const API_CHANNEL_SIZE: u32 = 50;

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
        let controller_handle = handle::ControllerHandle::new(sender);

        Ok(Self {
            caniot: CaniotDevicesController::new(iface, caniot_config, storage)?,
            copro: CoproController::new(copro_handle, controller_handle.clone())?,
            api_handle: controller_handle,
            api_receiver: receiver,
            shutdown,
            stats: ControllerCoreStats::default(),
        })
    }

    pub fn get_handle(&self) -> handle::ControllerHandle {
        self.api_handle.clone()
    }

    pub async fn run(mut self) -> Result<(), ()> {
        let _ = self.caniot.start().await;

        loop {
            let sys_now = Instant::now();
            let utc_now = Utc::now();

            let sleep_time = self.caniot.loop_process(&sys_now, &utc_now).await;

            select! {
                Some(frame) = self.caniot.iface.recv_poll() => {
                    self.caniot.handle_can_frame(frame).await;
                },
                message = self.copro.poll_message() => {
                    match message {
                        Some(msg) => self.copro.handle_message(msg).await,
                        None => {
                            error!("CoproController stream ended, shutting down");
                            error!("WHAT TO DO ?!?!?!");
                        }
                    }
                },
                Some(message) = self.api_receiver.recv() => {
                    let _ = self.handle_api_message(message).await;
                },
                _ = self.shutdown.recv() => {
                    warn!("Received shutdown signal, exiting ...");
                    break;
                }
                _ = sleep(sleep_time) => {
                    // Timeout of pending queries handled in handle_pending_queries_timeout()
                },
            }

            for event in self.caniot.pop_pending_sensor_change_events() {
                self.copro.notify_sensor_change_event(event).await;
            }

            self.stats.loop_runs += 1;
        }

        Ok(())
    }

    pub async fn handle_api_message(
        &mut self,
        message: ControllerApiMessage,
    ) -> Result<(), ControllerError> {
        self.stats.api_rx += 1;
        match message {
            ControllerApiMessage::GetStats { respond_to } => {
                let stats = ControllerStats {
                    caniot: self.caniot.stats,
                    core: self.stats,
                    can: self.caniot.iface.get_stats(),
                };
                let _ = respond_to.send(stats);
            }
            ControllerApiMessage::CaniotMessage(caniot_message) => {
                self.caniot.handle_api_message(caniot_message).await?;
            }
            ControllerApiMessage::CoprocessorMessage(copro_message) => {
                self.copro.handle_api_message(copro_message)?;
            }
        }

        Ok(())
    }
}
