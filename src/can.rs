use embedded_can::Frame as EmbeddedFrame;
use futures_util::StreamExt;
use rocket::fairing::Fairing;
use socketcan::tokio::CanSocket;
use socketcan::{CanDataFrame, CanFilter, CanFrame, Error as CanError, SocketOptions};
use thiserror::Error;

use log::{debug, error, info, warn};

use crate::caniot::{
    ConversionError, EmbeddedFrameWrapper, Id as CaniotId, Response as CaniotResponse,
    CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK,
};
use crate::config::CanConfig;
use crate::shared::{Shared, SharedHandle};
use crate::shutdown::Shutdown;

#[derive(Error, Debug)]
pub enum CanListenerError {
    #[error("SocketCAN error: {0}")]
    CanError(#[from] CanError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    // #[error("Conversion error: {0}")]
    // ConversionError(#[from] ConversionError),
}

fn handle_can_data_frame(frame: CanDataFrame, shared: &Shared) {
    let frame: Result<CaniotResponse, _> = EmbeddedFrameWrapper(frame).try_into();
    match frame {
        Ok(frame) => {
            shared.stats.lock().unwrap().can.rx += 1;
            info!("{}", frame);
        }
        Err(err) => {
            shared.stats.lock().unwrap().can.malformed += 1;
            error!("Failed to convert into CANIOT frame {}", err)
        }
    }
}

pub async fn can_listener(config: CanConfig, shared: SharedHandle) -> Result<(), CanListenerError> {
    let mut sock = CanSocket::open(&config.interface)?;

    // keep only CANIOT device frames
    let filter = CanFilter::new(CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK);
    sock.set_filters(&[filter])?;

    let mut shutdown = Shutdown::new(shared.notify_shutdown.subscribe());

    loop {
        tokio::select! {
            Some(res) = sock.next() => match res {
                Ok(CanFrame::Data(frame)) => handle_can_data_frame(frame, &shared),
                Ok(CanFrame::Remote(frame)) => warn!("Unhandled {:?}", frame),
                Ok(CanFrame::Error(frame)) => error!("{:?}", frame),
                Err(err) => error!("{}", err),
            },
            _ = shutdown.recv() => {
                warn!("Received shutdown signal");
                break;
            }
        }
    }

    Ok(())
}
