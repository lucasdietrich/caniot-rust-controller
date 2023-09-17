use env_logger;
use log;
use tokio;

use crate::{config, context, server, can};

pub fn init_controller() {
    env_logger::builder()
        .format_level(true)
        .format_target(true)
        .format_timestamp_millis()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Load the configuration
    let config = config::load_config();
    println!("AppConfig: {:?}", config);

    let context = context::new_context();

    // Initialize tokio runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        // .thread_name("my-custom-name")
        .enable_all()
        .build()
        .unwrap();

    let h_can = rt.spawn(can::can_socket(config.can, context.clone()));
    let h_rocket = rt.spawn(server::rocket(config.server, context.clone()).launch());

    rt.block_on(async {
        let _ = h_can.await.unwrap();
        let _ = h_rocket.await.unwrap();
    });
}