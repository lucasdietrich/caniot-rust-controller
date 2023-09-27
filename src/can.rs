use embedded_can::Frame as EmbeddedFrame;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use socketcan::tokio::CanSocket;
use socketcan::{CanDataFrame, CanFilter, CanFrame, Error as CanError, SocketOptions};
use thiserror::Error;
use tokio::sync::mpsc;

use crate::caniot::{
    ConversionError, EmbeddedFrameWrapper, Id as CaniotId, Request as CaniotRequest,
    Response as CaniotResponse, CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK,
};
use crate::shared::{Shared, SharedHandle};
use crate::shutdown::Shutdown;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CanConfig {
    pub interface: String,
}

impl Default for CanConfig {
    fn default() -> Self {
        CanConfig {
            interface: "vcan0".to_string(),
        }
    }
}

const CAN_TX_QUEUE_SIZE: usize = 10;

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
            info!("RX {}", frame);
        }
        Err(err) => {
            shared.stats.lock().unwrap().can.malformed += 1;
            error!("Failed to convert into CANIOT frame {}", err)
        }
    }
}

pub fn can_create_tx_queue() -> (mpsc::Sender<CaniotRequest>, mpsc::Receiver<CaniotRequest>) {
    mpsc::channel::<CaniotRequest>(CAN_TX_QUEUE_SIZE)
}

pub async fn can_listener(
    shared: SharedHandle,
    mut tx_queue_receiver: mpsc::Receiver<CaniotRequest>,
) -> Result<(), CanListenerError> {
    let config = &shared.config.can;
    let mut shutdown = Shutdown::new(shared.notify_shutdown.subscribe());
    let mut sock = CanSocket::open(&config.interface)?;

    // keep only CANIOT device frames
    let filter = CanFilter::new(CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK);
    sock.set_filters(&[filter])?;

    info!(
        "CAN listener started on {} with filter {:04x}:{:04x}",
        config.interface, CANIOT_DEVICE_FILTER_ID, CANIOT_DEVICE_FILTER_MASK
    );

    loop {
        tokio::select! {
            Some(res) = sock.next() => match res {
                Ok(CanFrame::Data(frame)) => handle_can_data_frame(frame, &shared),
                Ok(CanFrame::Remote(frame)) => warn!("Unhandled {:?}", frame),
                Ok(CanFrame::Error(frame)) => error!("{:?}", frame),
                Err(err) => error!("{}", err),
            },
            Some(request) = tx_queue_receiver.recv() => {
                info!("TX {}", request);
                shared.stats.lock().unwrap().can.tx += 1;
                let request: CanFrame = request.to_can_frame();
                sock.send(request).await?;
            },
            _ = shutdown.recv() => {
                warn!("Received shutdown signal");
                break;
            }
        }
    }

    Ok(())
}
