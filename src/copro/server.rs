use std::time::Duration;

use ble_copro_stream_server::{
    stream_channel::{StreamChannel, StreamChannelError},
    stream_message::ChannelMessage,
    xiaomi::XiaomiRecord,
    StreamServer, DEFAULT_LISTEN_PORT,
};
use log::{debug, error, warn};
use tokio::{sync::mpsc::Sender, time::sleep};

pub const LISTEN_IP: &str = "192.0.3.1";
pub const LISTEN_PORT: u16 = 4000;

pub async fn start(sender: Sender<XiaomiRecord>) {
    let server = StreamServer::init(LISTEN_IP, LISTEN_PORT)
        .await
        .expect("Failed to start server");

    tokio::spawn(async move {
        server_loop(server, sender).await;
    });
}

async fn server_loop(server: StreamServer, mut sender: Sender<XiaomiRecord>) {
    loop {
        match server.accept().await {
            Ok(mut channel) => loop {
                client_loop(&mut channel, &mut sender).await;
            },
            Err(e) => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn client_loop(channel: &mut StreamChannel, sender: &mut Sender<XiaomiRecord>) {
    loop {
        match channel.next().await {
            Ok(ChannelMessage::Xiaomi(record)) => {
                debug!("Received record: {:?}", record);

                let ret = sender.send(record).await;
                if let Err(e) = ret {
                    error!("Error sending record: {:?}", e);
                }
            }
            Ok(_) => {}
            Err(StreamChannelError::UnhandledChannelId) => {
                warn!("Unhandled channel ID");
            }
            Err(e) => {
                error!("Error reading message: {:?}", e);
                break;
            }
        }
    }
}
