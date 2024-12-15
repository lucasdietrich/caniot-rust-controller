use std::time::Duration;

use ble_copro_stream_server::{
    stream_channel::StreamChannel, stream_message::ChannelMessage, xiaomi::XiaomiRecord,
    StreamServer, Timestamp,
};
use chrono::Utc;
use rocket::error;
use tokio::{sync::mpsc, time::sleep};

use super::CoproConfig;

const COPRO_MSG_CHANNEL_SIZE: usize = 10;
const COPRO_SERVER_INIT_RETRY_INTERVAL: Duration = Duration::from_secs(5);
const COPRO_SERVER_ACCEPT_RETRY_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum CoproStreamChannelStatus {
    Disconnected,
    Connected,
    Error(String),
}

#[derive(Debug)]
pub enum CoproMessage {
    XiaomiRecord(XiaomiRecord),
    Status(CoproStreamChannelStatus),
}

pub struct Coprocessor {
    sender: mpsc::Sender<CoproMessage>,
    config: CoproConfig,
    state: State,
}

enum State {
    Uninitialized,
    Listening(StreamServer),
    Connected(StreamChannel, StreamServer),
}

pub struct CoproHandle {
    pub receiver: mpsc::Receiver<CoproMessage>,
}

impl Coprocessor {
    pub fn new(config: CoproConfig) -> (Self, CoproHandle) {
        let (sender, receiver) = mpsc::channel(COPRO_MSG_CHANNEL_SIZE);

        (
            Self {
                sender,
                config,
                state: State::Uninitialized,
            },
            CoproHandle { receiver },
        )
    }

    async fn notify_stream_channel_status(
        sender: &mut mpsc::Sender<CoproMessage>,
        status: CoproStreamChannelStatus,
    ) {
        let _ = sender.send(CoproMessage::Status(status)).await;
    }

    async fn run_state(
        state: State,
        sender: &mut mpsc::Sender<CoproMessage>,
        config: &CoproConfig,
    ) -> State {
        match state {
            State::Uninitialized => {
                match StreamServer::init(config.listen_ip.as_str(), config.listen_port).await {
                    Ok(server) => State::Listening(server),
                    Err(e) => {
                        error!("Failed to start server: {}", e);
                        Self::notify_stream_channel_status(
                            sender,
                            CoproStreamChannelStatus::Error("Failed to start server".to_string()),
                        )
                        .await;
                        sleep(COPRO_SERVER_INIT_RETRY_INTERVAL).await;
                        state
                    }
                }
            }
            State::Listening(server) => match server.accept().await.ok() {
                Some(channel) => {
                    Self::notify_stream_channel_status(sender, CoproStreamChannelStatus::Connected)
                        .await;
                    State::Connected(channel, server)
                }
                None => {
                    error!("Failed to accept connection");
                    Self::notify_stream_channel_status(
                        sender,
                        CoproStreamChannelStatus::Error("Failed to accept connection".to_string()),
                    )
                    .await;
                    sleep(COPRO_SERVER_ACCEPT_RETRY_INTERVAL).await;
                    State::Listening(server)
                }
            },
            State::Connected(mut client, server) => {
                while let Ok(ChannelMessage::Xiaomi(mut xiaomi_record)) = client.next().await {
                    // override xiaomi_record timestamp with current time
                    // TODO this needs to be changed to use Copro timestamp when available
                    xiaomi_record.timestamp = Timestamp::Utc(Utc::now());

                    let _ = sender.send(CoproMessage::XiaomiRecord(xiaomi_record)).await;
                }

                info!("Connection closed");

                Self::notify_stream_channel_status(sender, CoproStreamChannelStatus::Disconnected)
                    .await;
                State::Listening(server)
            }
        }
    }

    pub async fn run(mut self) {
        loop {
            self.state = Self::run_state(self.state, &mut self.sender, &self.config).await;
        }
    }
}
