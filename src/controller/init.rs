use std::sync::Arc;

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use super::Controller;
use crate::{bus::CanInterfaceTrait, config::AppConfig, shutdown::Shutdown};

pub fn init<IF: CanInterfaceTrait>(
    config: &AppConfig,
    rt: &Arc<Runtime>,
    notify_shutdown: &Sender<()>,
) -> Controller<IF> {
    let can_iface = rt.block_on(IF::new(&config.can)).unwrap();

    Controller::new(
        can_iface,
        config.caniot.clone(),
        Shutdown::new(notify_shutdown.subscribe()),
        rt.clone(),
    )
    .unwrap()
}
