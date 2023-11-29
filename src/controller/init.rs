use std::sync::Arc;

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use super::{Controller, DemoNode, Device};
use crate::{can, config::AppConfig, shutdown::Shutdown, caniot};


pub fn init<'a>(config: &AppConfig, rt: &Arc<Runtime>, notify_shutdown: &Sender<()>) -> Controller {

    let can_iface = rt.block_on(can::init_interface(&config.can));

    let demo = Box::new(Device::<DemoNode>::new(
        caniot::DeviceId::new(1).unwrap()
    ));

    Controller::new(
        can_iface,
        config.caniot.clone(),
        vec![
            demo,
        ],
        Shutdown::new(notify_shutdown.subscribe()),
        rt.clone(),
    ).unwrap()
}

