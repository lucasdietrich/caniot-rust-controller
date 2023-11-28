use std::sync::Arc;

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use super::Controller;
use crate::{can, config::AppConfig, shutdown::Shutdown};

pub fn init(config: &AppConfig, rt: &Arc<Runtime>, notify_shutdown: &Sender<()>) -> Controller {
    let can_iface = rt.block_on(can::init_interface(&config.can));

    Controller::new(
        can_iface,
        config.caniot.clone(),
        vec![],
        Shutdown::new(notify_shutdown.subscribe()),
        rt.clone(),
    )
}

