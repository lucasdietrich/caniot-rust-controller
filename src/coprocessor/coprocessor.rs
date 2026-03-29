use std::time::Duration;

use ble_copro_stream_server::{
    ble_control::{BleControlAction, BleControlPayload},
    device_control::{self, DeviceCtrlCommandMsg, DeviceCtrlStateMsg, GarageDoorsState},
    linky::LinkyTicRecord,
    stream_channel::StreamChannel,
    stream_message::ChannelMessage,
    xiaomi::XiaomiRecord,
    StreamServer, Timestamp,
};
use chrono::Utc;
use log::warn;
use rocket::error;
use tokio::{select, sync::mpsc, time::sleep};

use crate::controller::{
    copro_controller::CoproDeviceConfig,
    sensor_change_events::{GarageStateChanged, SensorChangeEvent},
    DoorState,
};

use super::CoproConfig;

const COPRO_MSG_TX_CHANNEL_SIZE: usize = 5;
const COPRO_MSG_RX_CHANNEL_SIZE: usize = 20;
const COPRO_SERVER_INIT_RETRY_INTERVAL: Duration = Duration::from_secs(5);
const COPRO_SERVER_ACCEPT_RETRY_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum CoproStreamChannelStatus {
    Disconnected,
    Connected,
    Error(String),
}

#[derive(Debug)]
pub enum RxCoproMessage {
    Xiaomi(XiaomiRecord),
    LinkyTic(LinkyTicRecord),
    Status(CoproStreamChannelStatus),
    BleControlEvent(BleControlPayload),
    DeviceControlCommand(DeviceCtrlCommandMsg),
}

#[derive(Debug)]
pub enum TxCoproMessage {
    RemoveBonds,
    EnablePairingAdv { duration_s: u32 },
    SensorChangeEvent(SensorChangeEvent),
}

pub struct Coprocessor {
    rx_queue: mpsc::Sender<RxCoproMessage>,
    tx_queue: mpsc::Receiver<TxCoproMessage>,
    listen_ip: String,
    listen_port: u16,
    state: State,
}

enum State {
    Uninitialized,
    Listening(StreamServer),
    Connected(StreamChannel, StreamServer),
}

pub struct CoproHandle {
    pub rx: mpsc::Receiver<RxCoproMessage>,
    pub tx: mpsc::Sender<TxCoproMessage>,
    pub devices_config: Vec<CoproDeviceConfig>,
}

// Driver, direct interaction with the BLE copro
impl Coprocessor {
    pub fn new(config: CoproConfig) -> (Self, CoproHandle) {
        let (rx_sender, rx_receiver) = mpsc::channel(COPRO_MSG_RX_CHANNEL_SIZE);
        let (tx_sender, tx_receiver) = mpsc::channel(COPRO_MSG_TX_CHANNEL_SIZE);
        (
            Self {
                rx_queue: rx_sender,
                tx_queue: tx_receiver,
                listen_ip: config.listen_ip,
                listen_port: config.listen_port,
                state: State::Uninitialized,
            },
            CoproHandle {
                rx: rx_receiver,
                tx: tx_sender,
                devices_config: config.devices,
            },
        )
    }

    async fn notify_stream_channel_status(
        sender: &mut mpsc::Sender<RxCoproMessage>,
        status: CoproStreamChannelStatus,
    ) {
        let _ = sender.send(RxCoproMessage::Status(status)).await;
    }

    async fn run_state(
        state: State,
        rxq_sender: &mut mpsc::Sender<RxCoproMessage>,
        txq_receiver: &mut mpsc::Receiver<TxCoproMessage>,
        listen_ip: &str,
        listen_port: u16,
    ) -> State {
        match state {
            State::Uninitialized => match StreamServer::init(listen_ip, listen_port).await {
                Ok(server) => State::Listening(server),
                Err(e) => {
                    error!("Failed to start Coprocessor server: {}", e);
                    Self::notify_stream_channel_status(
                        rxq_sender,
                        CoproStreamChannelStatus::Error(
                            "Failed to start Coprocessor server".to_string(),
                        ),
                    )
                    .await;
                    sleep(COPRO_SERVER_INIT_RETRY_INTERVAL).await;
                    state
                }
            },
            State::Listening(server) => match server.accept().await.ok() {
                Some(channel) => {
                    Self::notify_stream_channel_status(
                        rxq_sender,
                        CoproStreamChannelStatus::Connected,
                    )
                    .await;
                    State::Connected(channel, server)
                }
                None => {
                    error!("Failed to accept connection");
                    Self::notify_stream_channel_status(
                        rxq_sender,
                        CoproStreamChannelStatus::Error("Failed to accept connection".to_string()),
                    )
                    .await;
                    sleep(COPRO_SERVER_ACCEPT_RETRY_INTERVAL).await;
                    State::Listening(server)
                }
            },
            State::Connected(mut client, server) => {
                // Clear the tx queue to avoid processing old messages that were sent while the channel was disconnected
                while let Ok(_) = txq_receiver.try_recv() {}

                loop {
                    select! {
                        Some(tx_message) = txq_receiver.recv() => {
                            match tx_message {
                                TxCoproMessage::RemoveBonds => {
                                    log::info!("Sending remove bonds indication to copro");
                                    if let Err(e) = client.send_indication(BleControlAction::RemoveAllBonds).await {
                                        error!("Failed to send message to copro: {}", e);
                                        break;
                                    }
                                }
                                TxCoproMessage::EnablePairingAdv { duration_s } => {
                                    log::info!("Sending enable pairing adv indication to copro ({}s)", duration_s);
                                    if let Err(e) = client.send_indication(BleControlAction::EnablePairingAdv { duration_s }).await {
                                        error!("Failed to send enable pairing adv to copro: {}", e);
                                        break;
                                    }
                                }
                                TxCoproMessage::SensorChangeEvent(SensorChangeEvent::GarageStateChanged(event)) => {
                                    let indication: DeviceCtrlStateMsg = DeviceCtrlStateMsg::GarageDoors(event.into());
                                    log::info!("Sending garage state change indication to copro: {:?}", indication);
                                    if let Err(e) = client.send_indication(indication).await {
                                        error!("Failed to send message to copro: {}", e);
                                        break;
                                    }
                                }
                            }
                        },
                        result = client.next() => {
                            match result {
                                Ok(message) => match message {
                                    ChannelMessage::Xiaomi(mut xiaomi_record) => {
                                        // override xiaomi_record timestamp with current time
                                        // TODO this needs to be changed to use Copro timestamp when available
                                        xiaomi_record.timestamp = Timestamp::Utc(Utc::now());

                                        let _ = rxq_sender.send(RxCoproMessage::Xiaomi(xiaomi_record)).await;
                                    }
                                    ChannelMessage::LinkyTic(mut linky_tic_record) => {
                                        // override linky_tic_record timestamp with current time
                                        // TODO this needs to be changed to use Copro timestamp when available
                                        linky_tic_record.timestamp = Timestamp::Utc(Utc::now());

                                        let _ = rxq_sender
                                            .send(RxCoproMessage::LinkyTic(linky_tic_record))
                                            .await;
                                    }
                                    ChannelMessage::BleControl(ble_event) => {
                                        let _ = rxq_sender
                                            .send(RxCoproMessage::BleControlEvent(ble_event))
                                            .await;
                                    }
                                    ChannelMessage::DeviceControl(command) => {
                                        let _ = rxq_sender
                                            .send(RxCoproMessage::DeviceControlCommand(command))
                                            .await;
                                    }
                                    _ => {
                                        warn!("Unhandled channel message: {:?}", message);
                                    }
                                },
                                Err(err) => {
                                    error!("Error reading from stream channel: {}", err);
                                    break;
                                }
                            }
                        },
                    }
                }

                info!("Connection closed");

                Self::notify_stream_channel_status(
                    rxq_sender,
                    CoproStreamChannelStatus::Disconnected,
                )
                .await;
                State::Listening(server)
            }
        }
    }

    pub async fn run(mut self) {
        loop {
            self.state = Self::run_state(
                self.state,
                &mut self.rx_queue,
                &mut self.tx_queue,
                &self.listen_ip,
                self.listen_port,
            )
            .await;
        }
    }
}

impl Into<device_control::DoorState> for DoorState {
    fn into(self) -> device_control::DoorState {
        match self {
            DoorState::Open => device_control::DoorState::Open,
            DoorState::Closed => device_control::DoorState::Closed,
        }
    }
}

impl Into<GarageDoorsState> for GarageStateChanged {
    fn into(self) -> GarageDoorsState {
        GarageDoorsState {
            left_door: self.left_door_state.into(),
            gate: self.gate_state.into(),
            right_door: self.right_door_state.into(),
        }
    }
}
