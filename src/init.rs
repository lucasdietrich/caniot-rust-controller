use std::sync::Arc;

use log::info;
use tokio::sync::broadcast;
use tokio::{self};

use crate::database::Storage;
use crate::internal::firmware::FirmwareInfos;
use crate::internal::software::SoftwareInfos;
use crate::shared::Shared;
use crate::{bus, config, controller, logger, webserver};

use crate::grpcserver;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: Option<String>,
}

fn get_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
}

pub fn run_controller() {
    let firmware_infos = FirmwareInfos::default();
    let software_infos = SoftwareInfos::default();

    logger::init_logger();

    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let config_file = args.config.as_deref().map(|x| x.trim());
    let config = config::load_config(config_file);

    debug!("Config: {:#?}", config);

    let rt = get_tokio_rt();
    let rt = Arc::new(rt);
    let (notify_shutdown, _) = broadcast::channel(1);

    let storage = rt
        .block_on(Storage::try_connect(&config.database))
        .expect("Failed to connect to database");
    rt.block_on(storage.initialize_tables())
        .expect("Failed to initialize database tables");
    let storage_handle = Arc::new(storage);

    let controller =
        controller::init::<bus::IFaceType>(&rt, &config, &storage_handle, &notify_shutdown);

    let shared = Arc::new(Shared::new(
        &rt,
        Arc::new(controller.get_handle()),
        &storage_handle,
        &config,
        notify_shutdown.clone(),
        firmware_infos,
        software_infos,
    ));

    let h_ctrl = rt.spawn(controller.run());
    let h_rocket = rt.spawn(webserver::rocket_server(shared.clone()));
    let h_grpc = rt.spawn(grpcserver::grpc_server(shared.clone()));

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

    let _ = rt.block_on(async { tokio::join!(h_ctrl, h_rocket, h_grpc) });
    println!("Exiting...");
}
