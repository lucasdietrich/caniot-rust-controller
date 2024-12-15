use std::time::Duration;

use ble_copro_stream_server::{
    stream_message::ChannelMessage, xiaomi::XiaomiRecord, StreamServer, Timestamp,
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
    Disconnected,
    Connected(StreamServer),
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
                state: State::Disconnected,
            },
            CoproHandle { receiver },
        )
    }

    async fn notify_stream_channel_status(&mut self, status: CoproStreamChannelStatus) {
        let _ = self.sender.send(CoproMessage::Status(status)).await;
    }

    pub async fn run(mut self) {
        loop {
            match self.state {
                State::Disconnected => {
                    match StreamServer::init(
                        self.config.listen_ip.as_str(),
                        self.config.listen_port,
                    )
                    .await
                    {
                        Ok(server) => {
                            self.state = State::Connected(server);
                            self.notify_stream_channel_status(CoproStreamChannelStatus::Connected)
                                .await;
                        }
                        Err(e) => {
                            error!("Failed to start server: {}", e);
                            self.notify_stream_channel_status(CoproStreamChannelStatus::Error(
                                "Failed to start server".to_string(),
                            ));
                            sleep(COPRO_SERVER_INIT_RETRY_INTERVAL).await;
                        }
                    }
                }
                State::Connected(ref server) => {
                    if let Some(mut client) = server.accept().await.ok() {
                        info!("Coprocessor stream accepted");

                        while let Ok(ChannelMessage::Xiaomi(mut xiaomi_record)) =
                            client.next().await
                        {
                            // override xiaomi_record timestamp with current time
                            // TODO this needs to be changed to use Copro timestamp when available
                            xiaomi_record.timestamp = Timestamp::Utc(Utc::now());

                            let _ = self
                                .sender
                                .send(CoproMessage::XiaomiRecord(xiaomi_record))
                                .await;
                        }

                        info!("Connection closed");

                        self.state = State::Disconnected;
                        self.notify_stream_channel_status(CoproStreamChannelStatus::Connected)
                            .await;
                    } else {
                        error!("Failed to accept connection");
                        self.notify_stream_channel_status(CoproStreamChannelStatus::Error(
                            "Failed to accept connection".to_string(),
                        ))
                        .await;
                        sleep(COPRO_SERVER_ACCEPT_RETRY_INTERVAL).await;
                    }
                }
            }
        }
    }
}
