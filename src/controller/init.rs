use std::sync::Arc;

use tokio::{
    runtime::Runtime,
    sync::{broadcast::Sender, RwLock},
};

use super::Controller;
use crate::{bus::CanInterfaceTrait, config::AppConfig, database::Database, shutdown::Shutdown};

pub fn init<IF: CanInterfaceTrait>(
    rt: &Arc<Runtime>,
    config: &AppConfig,
    _db: &Arc<RwLock<Database>>,
    notify_shutdown: &Sender<()>,
) -> Controller<IF> {
    let can_iface = rt
        .block_on(IF::new(&config.can))
        .expect("Failed to create CAN interface");

    Controller::new(
        can_iface,
        config.caniot.clone(),
        Shutdown::new(notify_shutdown.subscribe()),
    )
    .expect("Failed to create controller")
}
