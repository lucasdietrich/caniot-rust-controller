use std::sync::Arc;

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use super::{Controller};
use crate::{bus::CanInterface, config::AppConfig, shutdown::Shutdown};
// use super::device::DeviceTrait;

pub fn init<'a>(config: &AppConfig, rt: &Arc<Runtime>, notify_shutdown: &Sender<()>) -> Controller {
    let can_iface = rt.block_on(async { CanInterface::new(&config.can).await.unwrap() });

    Controller::new(
        can_iface,
        config.caniot.clone(),
        // vec![
        //     demo,
        // ],
        Shutdown::new(notify_shutdown.subscribe()),
        rt.clone(),
    )
    .unwrap()
}
