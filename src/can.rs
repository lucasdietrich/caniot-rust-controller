use rocket::fairing::Fairing;
use socketcan::{CanFrame, Error, Result, CanDataFrame};
use socketcan::tokio::CanSocket;
use futures_util::StreamExt;

use log::{debug, info, warn, error};

use crate::config::CanConfig;
use crate::context::ContextHandle;

pub async fn can_socket(config: CanConfig, context: ContextHandle) -> Result<()> {
    let mut sock = CanSocket::open(&config.interface)?;

    while let Some(res) = sock.next().await {
        match res {
            Ok(CanFrame::Data(frame)) => {
                info!("Received {:?}", frame);
                context.lock().unwrap().stats.can.rx += 1;
            },
            Ok(CanFrame::Remote(frame)) => warn!("Unhandled {:?}", frame),
            Ok(CanFrame::Error(frame)) => error!("{:?}", frame),
            Err(err) => eprintln!("{}", err),
        }
    }

    Ok(())
}