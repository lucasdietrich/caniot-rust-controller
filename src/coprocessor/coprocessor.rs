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

pub enum CoproMessage {
    XiaomiRecord(XiaomiRecord),
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
                        }
                        Err(e) => {
                            eprintln!("Failed to start server: {}", e);
                            sleep(COPRO_SERVER_INIT_RETRY_INTERVAL).await;
                        }
                    }
                }
                State::Connected(ref server) => {
                    if let Some(mut client) = server.accept().await.ok() {
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
                    } else {
                        error!("Failed to accept connection");
                        sleep(COPRO_SERVER_ACCEPT_RETRY_INTERVAL).await;
                    }
                }
            }
        }
    }
}
