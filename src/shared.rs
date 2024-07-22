use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{broadcast, RwLock};

use serde::Serialize;

use crate::config::AppConfig;
use crate::controller::ControllerHandle;
use crate::database::Database;
use crate::internal::firmware::FirmwareInfos;
use crate::internal::software::SoftwareInfos;

pub type SharedHandle = Arc<Shared>;

/// The `Shared` struct contains fields for managing shared state between asynchronous tasks.
#[derive(Debug)]
pub struct Shared {
    /// The Tokio runtime
    pub rt: Arc<Runtime>,

    /// The CAN controller handle
    pub controller_handle: Arc<ControllerHandle>,

    /// The database handle
    pub db: Arc<RwLock<Database>>,

    /// The application configuration
    pub config: AppConfig,

    /// Used to signal the asynchronous task to shutdown
    /// The task subscribes to this channel
    pub notify_shutdown: broadcast::Sender<()>,

    // TODO: Move this out of the structure, or create an Arc, because this gets cloned a lot
    // Firmware infos
    pub firmware_infos: FirmwareInfos,

    // TODO: Move this out of the structure, or create an Arc, because this gets cloned a lot
    // Software infos
    pub software_infos: SoftwareInfos,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct ServerStats {}

impl Shared {
    pub fn new(
        rt_handle: &Arc<Runtime>,
        controller_handle: Arc<ControllerHandle>,
        db_handle: &Arc<RwLock<Database>>,
        config: &AppConfig,
        notify_shutdown: broadcast::Sender<()>,
        firmware_infos: FirmwareInfos,
        software_infos: SoftwareInfos,
    ) -> Self {
        Shared {
            rt: rt_handle.clone(),
            controller_handle: controller_handle,
            db: db_handle.clone(),
            config: config.clone(),
            notify_shutdown,
            firmware_infos,
            software_infos,
        }
    }
}
