use std::sync::Arc;

use log::info;
use tokio::sync::broadcast;
use tokio::{self};


use crate::{config, controller, logger, shared, webserver};

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

#[cfg(feature = "emu")]
type IFaceType = crate::bus::emu::CanInterface;
#[cfg(not(feature = "emu"))]
type IFaceType = crate::bus::can::CanInterface;

pub fn run_controller() {
    logger::init_logger();

    let config = config::load_config();
    println!("AppConfig: {:#?}", config);

    let rt = get_tokio_rt();
    let rt = Arc::new(rt);
    let (notify_shutdown, _) = broadcast::channel(1);

    // let database = rt
    //     .block_on(Database::new(&config.database.connection_string))
    //     .unwrap();

    let controller = controller::init::<IFaceType>(&config, &rt, &notify_shutdown);

    let shared = shared::new_context(
        rt.clone(),
        Arc::new(controller.get_handle()),
        &config,
        notify_shutdown.clone(),
    );

    rt.spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        info!("CTRL+C received, shutting down...");

        // Mark shutdown as requested
        let _ = notify_shutdown.send(());

        // drop notify_shutdown to signal the shutdown
        drop(notify_shutdown);
    });

    let h_ctrl = rt.spawn(controller.run());
    let h_rocket = rt.spawn(webserver::rocket(shared.clone()).launch());

    #[cfg(feature = "grpc")]
    let h_grpc = rt.spawn(grpcserver::grpc_server(shared.clone()));

    #[cfg(feature = "grpc")]
    let _ = rt.block_on(async { tokio::join!(h_ctrl, h_rocket, h_grpc) });

    #[cfg(not(feature = "grpc"))]
    let _ = rt.block_on(async { tokio::join!(h_ctrl, h_rocket) });
}
