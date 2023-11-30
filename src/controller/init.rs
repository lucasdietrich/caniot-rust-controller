use std::sync::Arc;

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use super::{Controller, DemoNode};
use crate::{can, config::AppConfig, shutdown::Shutdown, caniot};
// use super::device::DeviceTrait;


pub fn init<'a>(config: &AppConfig, rt: &Arc<Runtime>, notify_shutdown: &Sender<()>) -> Controller {

    let can_iface = rt.block_on(can::init_interface(&config.can));

    Controller::new(
        can_iface,
        config.caniot.clone(),
        // vec![
        //     demo,
        // ],
        Shutdown::new(notify_shutdown.subscribe()),
        rt.clone(),
    ).unwrap()
}

