use embedded_can::Frame as EmbeddedFrame;
use futures_util::StreamExt;
use rocket::fairing::Fairing;
use socketcan::tokio::CanSocket;
use socketcan::{CanDataFrame, CanFilter, CanFrame, Error as CanError, SocketOptions};
use thiserror::Error;

use log::{debug, error, info, warn};

use crate::caniot::{
    ConversionError, EmbeddedFrameWrapper, Frame as CaniotFrame, Id as CaniotId,
    CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK,
};
use crate::config::CanConfig;
use crate::context::ContextHandle;

#[derive(Error, Debug)]
pub enum CanListenerError {
    #[error("SocketCAN error: {0}")]
    CanError(#[from] CanError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    // #[error("Conversion error: {0}")]
    // ConversionError(#[from] ConversionError),
}

pub async fn can_listener(
    config: CanConfig,
    context: ContextHandle,
) -> Result<(), CanListenerError> {
    let mut sock = CanSocket::open(&config.interface)?;

    // keep only CANIOT device frames
    let filter = CanFilter::new(CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK);
    sock.set_filters(&[filter])?;

    while let Some(res) = sock.next().await {
        match res {
            Ok(CanFrame::Data(frame)) => {
                let frame: Result<CaniotFrame, _> = EmbeddedFrameWrapper(frame).try_into();
                match frame {
                    Ok(frame) => {
                        context.lock().unwrap().stats.can.rx += 1;
                        debug!("Received {:?}", frame);
                    }
                    Err(err) => {
                        context.lock().unwrap().stats.can.malformed += 1;
                        error!("Failed to convert to CANIOT frame {}", err)
                    }
                }
            }
            Ok(CanFrame::Remote(frame)) => warn!("Unhandled {:?}", frame),
            Ok(CanFrame::Error(frame)) => error!("{:?}", frame),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}
