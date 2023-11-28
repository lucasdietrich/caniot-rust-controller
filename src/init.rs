use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::sync::broadcast;
use tokio::{self, time::sleep};

use crate::shutdown::Shutdown;
use crate::{can, caniot, config, controller, logger, shared, webserver};

#[cfg(feature = "grpc")]
use crate::grpcserver;

fn get_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        // .thread_name("my-custom-name")
        .enable_all()
        .build()
        .unwrap()
}

pub fn run_controller() {
    logger::init_logger();

    let config = config::load_config();
    println!("AppConfig: {:?}", config);

    let (notify_shutdown, _) = broadcast::channel(1);

    let rt = get_tokio_rt();
    let rt = Arc::new(rt);

    let can_iface = rt.block_on(can::init_interface(&config.can));
    let caniot_controller = controller::Controller::new(
        can_iface,
        Shutdown::new(notify_shutdown.subscribe()),
        rt.clone(),
    );
    let caniot_controller_handle = caniot_controller.get_handle();

    let shared = shared::new_context(
        rt.clone(),
        Arc::new(caniot_controller_handle),
        &config,
        notify_shutdown.clone(),
    );

    rt.spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        info!("CTRL+C received, shutting down...");

        let _ = notify_shutdown.send(());
    });

    let h_ctrl = rt.spawn(caniot_controller.run());
    let h_rocket = rt.spawn(webserver::rocket(shared.clone()).launch());

    #[cfg(feature = "grpc")]
    let h_grpc = rt.spawn(grpcserver::grpc_server(shared.clone()));

    #[cfg(feature = "grpc")]
    let _ = rt.block_on(async { tokio::join!(h_ctrl, h_rocket, h_grpc) });

    #[cfg(not(feature = "grpc"))]
    let _ = rt.block_on(async { tokio::join!(h_ctrl, h_rocket) });
}
