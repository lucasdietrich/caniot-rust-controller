use std::sync::Arc;

use tokio::{runtime::Runtime, sync::broadcast::Sender};

use crate::{bus::CanInterfaceTrait, config::AppConfig, database::Storage, shutdown::Shutdown};

use super::controller::Controller;

pub fn init<IF: CanInterfaceTrait>(
    rt: &Arc<Runtime>,
    config: &AppConfig,
    storage: &Arc<Storage>,
    notify_shutdown: &Sender<()>,
) -> Controller<IF> {
    let can_iface = rt
        .block_on(IF::new(&config.can))
        .expect("Failed to create CAN interface");

    #[cfg(feature = "ble-copro")]
    let (coprocessor, copro_handle) = crate::coprocessor::Coprocessor::new(config.copro.clone());

    #[cfg(feature = "ble-copro")]
    rt.spawn(coprocessor.run());

    Controller::new(
        can_iface,
        config.caniot.clone(),
        #[cfg(feature = "ble-copro")]
        copro_handle,
        storage.clone(),
        Shutdown::new(notify_shutdown.subscribe()),
    )
    .expect("Failed to create controller")
}
