use futures_util::Future;
use log;
use rocket::fairing::Fairing;
use tokio;
use tokio::sync::{broadcast, mpsc};

use crate::{can, config, logger, server, shared};

fn get_tokio_rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        // .thread_name("my-custom-name")
        .enable_all()
        .build()
        .unwrap()
}

pub fn init_controller() {
    logger::init_logger();

    let config = config::load_config();
    println!("AppConfig: {:?}", config);

    let (notify_shutdown, _) = broadcast::channel(1);

    let (can_q_sender, can_q_receiver) = can::can_create_tx_queue();
    let shared = shared::new_context(notify_shutdown.clone(), can_q_sender);

    // Initialize tokio runtime
    let rt = get_tokio_rt();

    rt.spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        let _ = notify_shutdown.send(());
    });

    let h_can = rt.spawn(can::can_listener(
        config.can,
        shared.clone(),
        can_q_receiver,
    ));
    let h_rocket = rt.spawn(server::rocket(config.server, shared.clone()).launch());

    let _ = rt.block_on(async { tokio::join!(h_can, h_rocket) });
}
