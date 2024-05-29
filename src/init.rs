use std::sync::Arc;

use log::info;
use tokio::sync::{broadcast, RwLock};
use tokio::{self};

use crate::database::Database;
use crate::shared::Shared;
use crate::{bus, config, controller, logger, webserver};

use crate::grpcserver;

fn get_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
}

pub fn run_controller() {
    logger::init_logger();

    let config = config::load_config();
    println!("AppConfig: {:#?}", config);

    let rt = get_tokio_rt();
    let rt = Arc::new(rt);
    let (notify_shutdown, _) = broadcast::channel(1);

    let database = rt
        .block_on(Database::new(&config.database.connection_string))
        .expect("Failed to connect to database");
    rt.block_on(database.initialize_tables())
        .expect("Failed to initialize database tables");
    let database_handle = Arc::new(RwLock::new(database));

    // read settings from database
    let settings_lg = rt.block_on(database_handle.read());
    let settings = settings_lg.get_settings();
    let iface_type = if let Some(iface_type) = rt.block_on(settings.get("iface_type")) {
        iface_type
    } else {
        rt.block_on(settings.set("iface_type", "can"))
            .expect("Failed to set iface_type");
        "can".to_string()
    };
    info!("iface_type: {}", iface_type);
    drop(settings_lg);

    let controller =
        controller::init::<bus::IFaceType>(&rt, &config, &database_handle, &notify_shutdown);

    let shared = Arc::new(Shared::new(
        &rt,
        Arc::new(controller.get_handle()),
        &database_handle,
        &config,
        notify_shutdown.clone(),
    ));

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
    let h_grpc = rt.spawn(grpcserver::grpc_server(shared));

    let _ = rt.block_on(async { tokio::join!(h_ctrl, h_rocket, h_grpc) });
}
