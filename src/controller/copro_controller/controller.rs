use ble_copro_stream_server::{
    ble::BleAddress,
    ble_control::{BleControlMessage, BleControlPayload, ConnectionEvent, PairingEvent},
    control_channel::{ControlMessage, FirmwareVersion},
    device_control::{DeviceCtrlCmd, DeviceCtrlCommandMsg},
    linky::LinkyTicRecord,
    xiaomi::XiaomiRecord,
};
use itertools::Itertools;

use crate::{
    controller::{
        copro_controller::{
            measurements::{
                BatteryTrait, BleEnergyMeterMeasurement, BleEnvironementalMeasurement, RssiTrait,
            },
            sensor::BleSersorType,
        },
        filtering::SensorFilter,
        sensor_change_events::SensorChangeEvent,
        GarageAction, GarageDoorCommand, SensorAlert,
    },
    coprocessor::{
        coprocessor::{CoproStreamChannelStatus, TxCoproMessage},
        CoproHandle, RxCoproMessage,
    },
    utils::{PrometheusExporterTrait, PrometheusNoLabel},
};

use chrono::{DateTime, Utc};
use log::{error, info, warn};
use thiserror::Error;

use crate::controller::handle::ControllerHandle;

use super::{api_message::CoproApiMessage, sensor::BleSensor};

pub struct CoproController {
    // Interaction with Coprocessor task
    handle: CoproHandle,

    // Handle for sending actions back to the main controller
    controller_handle: ControllerHandle,

    // Current context
    sensors: Vec<BleSensor>,
    copro_status: CoproStreamChannelStatus,
    stats: CoproControllerStats,

    // Connection/pairing/bonding state
    devices: Vec<BleDevice>,

    // Pairing advertising state
    pairing_adv_active: bool,
    pairing_adv_duration_s: u32,

    // BLE Coprocessor firmware version (received on each connection)
    firmware_version: Option<FirmwareVersion>,
}

#[derive(Debug, Error)]
pub enum CoproError {}

#[derive(Debug, Default, Clone)]
pub struct CoproControllerStats {
    pub rx_packets: u64,
    pub rx_type_xiaomi: u64,
    pub rx_type_linky_tic: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BleDevicePairingState {
    None,
    Pending { code: u32 },
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct BleDeviceStats {
    pub connection_events: u64,
    pub pairing_events: u64,
    pub commands_received: u64,
}

#[derive(Debug, Clone)]
pub struct BleDevice {
    pub addr: BleAddress,
    pub connected: bool,
    pub pairing: BleDevicePairingState,
    pub stats: BleDeviceStats,
    pub last_seen: DateTime<Utc>,
}

impl<'a> PrometheusExporterTrait<'a> for CoproControllerStats {
    type Label = PrometheusNoLabel;

    fn export(&self, _labels: impl AsRef<[&'a Self::Label]>) -> String {
        format!(
            "controller_copro_iface_rx {}\n\
            controller_copro_iface_rx {{type=\"xiaomi\"}} {}\n\
            controller_copro_iface_rx {{type=\"linky_tic\"}} {}\n\
            ",
            self.rx_packets, self.rx_type_xiaomi, self.rx_type_linky_tic,
        )
    }
}

impl CoproController {
    pub fn new(
        handle: CoproHandle,
        controller_handle: ControllerHandle,
    ) -> Result<CoproController, CoproError> {
        Ok(CoproController {
            handle,
            controller_handle,
            sensors: Vec::new(),
            copro_status: CoproStreamChannelStatus::Disconnected,
            stats: CoproControllerStats::default(),
            devices: Vec::new(),
            pairing_adv_active: false,
            pairing_adv_duration_s: 0,
            firmware_version: None,
        })
    }

    pub async fn poll_message(&mut self) -> Option<RxCoproMessage> {
        self.handle.rx.recv().await
    }

    async fn handle_xiaomi_message(&mut self, record: XiaomiRecord) {
        info!("ble xiaomi {}", record);
        self.stats.rx_packets += 1;
        self.stats.rx_type_xiaomi += 1;

        let record_timestamp = record.timestamp.to_utc().unwrap_or(Utc::now());

        if let Some(device) = self
            .sensors
            .iter_mut()
            .find(|d| d.ble_addr == record.ble_addr)
        {
            let _ = device.commit_new_environemental_measurement(record_timestamp, record);
        } else {
            let device_config = self
                .handle
                .devices_config
                .iter()
                .find(|config| config.mac == record.ble_addr.mac_string());

            // Name / location
            let name = device_config
                .map(|config| config.name.clone())
                .unwrap_or_else(|| {
                    BleSensor::default_name(&BleSersorType::Xiaomi, &record.ble_addr)
                });
            let location = device_config.and_then(|config| config.location.clone());

            // Instantiate device
            let mut device = BleSensor::init(
                record.ble_addr,
                name,
                BleSersorType::Xiaomi,
                record_timestamp,
                record.rssi(),
                record.battery_mv(),
                record.battery_level(),
                BleEnvironementalMeasurement::init(record),
                location,
            );

            // UI display order (defaults to 0 if not provided)
            let ui_display_order = device_config
                .map(|config| config.ui_display_order)
                .unwrap_or(0);
            device.set_ui_display_order(ui_display_order);

            info!("new device: {:?}", device);
            self.sensors.push(device);
        }
    }

    async fn handle_linky_tic_message(&mut self, record: LinkyTicRecord) {
        info!("ble linky tic {}", record);
        self.stats.rx_packets += 1;
        self.stats.rx_type_linky_tic += 1;

        let record_timestamp = record.timestamp.to_utc().unwrap_or(Utc::now());

        if let Some(device) = self
            .sensors
            .iter_mut()
            .find(|d| d.sensor_type == BleSersorType::LinkyTIC)
        {
            // TODO, how to handle a rebooting TIC device, or multiple TIC devices ?
            if device.ble_addr != record.ble_addr {
                warn!(
                    "linky tic device address changed from {} to {}, updating ...",
                    device.ble_addr, record.ble_addr
                );
                device.ble_addr = record.ble_addr;
            }

            let _ = device.commit_new_energy_meter_measurement(record_timestamp, record);
        } else {
            // Instantiate device
            let mut device = BleSensor::init(
                record.ble_addr,
                "Linky TIC".to_string(),
                BleSersorType::LinkyTIC,
                record_timestamp,
                record.rssi(),
                None,
                None,
                BleEnergyMeterMeasurement::init(record),
                Some("Garage".to_string()),
            );

            // Set first
            device.set_ui_display_order(0);

            info!("new device: {:?}", device);
            self.sensors.push(device);
        }
    }

    fn get_or_insert_ble_device(&mut self, addr: BleAddress) -> &mut BleDevice {
        let now = Utc::now();
        if let Some(pos) = self.devices.iter().position(|d| d.addr == addr) {
            self.devices[pos].last_seen = now;
            &mut self.devices[pos]
        } else {
            self.devices.push(BleDevice {
                addr,
                connected: false,
                pairing: BleDevicePairingState::None,
                stats: BleDeviceStats::default(),
                last_seen: now,
            });
            self.devices.last_mut().unwrap()
        }
    }

    async fn handle_ble_control_message(&mut self, event: BleControlPayload) {
        info!("BLE event: {}", event);

        match event.message {
            BleControlMessage::Connection(ConnectionEvent::Connected) => {
                let device = self.get_or_insert_ble_device(event.addr);

                device.connected = true;
                device.stats.connection_events += 1;
            }
            BleControlMessage::Connection(ConnectionEvent::Disconnected) => {
                // remove device if not paired, otherwise keep it with connected = false (to keep pairing state and stats)
                if let Some(pos) = self.devices.iter().position(|d| d.addr == event.addr) {
                    if self.devices[pos].pairing == BleDevicePairingState::None {
                        self.devices.remove(pos);
                    } else {
                        self.devices[pos].connected = false;
                        self.devices[pos].last_seen = Utc::now();
                    }
                }
            }
            BleControlMessage::Pairing(PairingEvent::PairingCode { code }) => {
                let device = self.get_or_insert_ble_device(event.addr);

                device.pairing = BleDevicePairingState::Pending { code };
                device.stats.pairing_events += 1;
            }
            BleControlMessage::Pairing(PairingEvent::PairingCancelled) => {
                if let Some(dev) = self.devices.iter_mut().find(|d| d.addr == event.addr) {
                    dev.pairing = BleDevicePairingState::Failed;
                    dev.last_seen = Utc::now();
                }
            }
            BleControlMessage::Pairing(PairingEvent::PairingSucceeded) => {
                self.get_or_insert_ble_device(event.addr).pairing =
                    BleDevicePairingState::Succeeded;
            }
            BleControlMessage::Pairing(PairingEvent::AllBondsRemoved) => {
                self.devices.clear();
            }
            BleControlMessage::Pairing(PairingEvent::PairingAdvStarted { duration_s }) => {
                info!("Pairing advertising started for {} seconds", duration_s);
                self.pairing_adv_active = true;
                self.pairing_adv_duration_s = duration_s;
            }
            BleControlMessage::Pairing(PairingEvent::PairingAdvStopped) => {
                info!("Pairing advertising stopped");
                self.pairing_adv_active = false;
                self.pairing_adv_duration_s = 0;
            }
            // Update device address to identity address
            BleControlMessage::IdentityResolved { rpa, identity } => {
                if let Some(pos) = self.devices.iter().position(|d| d.addr == rpa) {
                    self.devices[pos].addr = identity;
                    self.devices[pos].last_seen = Utc::now();
                }
            }
        }

        info!("BLE devices state: {:?}", self.devices);
    }

    async fn handle_device_control_command(&mut self, command: DeviceCtrlCommandMsg) {
        info!("Received DeviceControl command from copro: {:?}", command);

        // Update stats
        self.devices
            .iter_mut()
            .find(|d| d.addr == command.ble_addr)
            .map(|device| {
                device.stats.commands_received += 1;
            });

        let garage_door_command = match command.cmd {
            DeviceCtrlCmd::OpenLeftGarageDoor => GarageDoorCommand::OPEN_LEFT,
            DeviceCtrlCmd::OpenRightGarageDoor => GarageDoorCommand::OPEN_RIGHT,
        };

        // We should use the no_response version of the API here,
        // as the await is blocking the processing of other incoming messages from the copro
        self.controller_handle
            .caniot_device_action_inner_no_response(
                None,
                GarageAction::SetStatus(garage_door_command),
            )
            .await;
    }

    async fn handle_control_message(&mut self, control_msg: ControlMessage) {
        match control_msg {
            ControlMessage::FirmwareVersion(version) => {
                info!("BLE Coprocessor firmware version: {}", version);
                self.firmware_version = Some(version);
            }
        }
    }

    pub async fn handle_message(&mut self, message: RxCoproMessage) {
        match message {
            RxCoproMessage::Control(control_msg) => self.handle_control_message(control_msg).await,
            RxCoproMessage::Xiaomi(record) => self.handle_xiaomi_message(record).await,
            RxCoproMessage::LinkyTic(record) => self.handle_linky_tic_message(record).await,
            RxCoproMessage::BleControlEvent(event) => self.handle_ble_control_message(event).await,
            RxCoproMessage::DeviceControlCommand(command) => {
                self.handle_device_control_command(command).await
            }
            RxCoproMessage::Status(status) => {
                info!("Coprocessor status changed: {:?}", status);
                self.copro_status = status;
            }
        }
    }

    fn get_controller_alert(&self) -> Option<SensorAlert> {
        match self.copro_status {
            CoproStreamChannelStatus::Error(ref msg) => Some(SensorAlert::new_error(msg)),
            CoproStreamChannelStatus::Disconnected => Some(SensorAlert::new_warning(
                "BLE Coprocessor dongle undetected",
            )),
            CoproStreamChannelStatus::Connected => {
                let alert = SensorAlert::new_ok("BLE Coprocessor dongle connected");
                Some(match &self.firmware_version {
                    Some(v) => alert.with_description(&format!("Version firmware : {}", v)),
                    None => alert,
                })
            }
        }
    }

    fn request_remove_bonds(&mut self) {
        if let Err(err) = self.handle.tx.try_send(TxCoproMessage::RemoveBonds) {
            error!("Failed to send remove bonds message to copro: {}", err);
        }
    }

    fn request_enable_pairing_adv(&mut self, duration_s: u32) {
        if let Err(err) = self
            .handle
            .tx
            .try_send(TxCoproMessage::EnablePairingAdv { duration_s })
        {
            error!(
                "Failed to send enable pairing adv message to copro: {}",
                err
            );
        }
    }

    // Return a list of devices with given filter
    fn get_devices(&self, filter: SensorFilter) -> Vec<BleSensor> {
        let filter_function = filter.get_filter_function::<BleSensor>();
        let sort_function = filter.get_sort_function::<BleSensor>();
        self.sensors
            .iter()
            .filter(|device| filter_function(device))
            .sorted_by(|a, b| sort_function(a, b))
            .cloned()
            .collect()
    }

    pub async fn notify_sensor_change_event(&mut self, event: SensorChangeEvent) {
        if let Err(err) = self
            .handle
            .tx
            .send(TxCoproMessage::SensorChangeEvent(event))
            .await
        {
            error!("Failed to send sensor change event to copro: {}", err);
        }
    }

    pub fn handle_api_message(&mut self, message: CoproApiMessage) -> Result<(), CoproError> {
        match message {
            CoproApiMessage::GetDevices { respond_to, filter } => {
                let devices = self.get_devices(filter);
                respond_to.send(devices).ok();
            }
            CoproApiMessage::GetAlert { respond_to } => {
                respond_to.send(self.get_controller_alert()).ok();
            }
            CoproApiMessage::GetStats { respond_to } => {
                respond_to.send(self.stats.clone()).ok();
            }
            CoproApiMessage::ResetDevicesMeasuresStats => {
                for device in self.sensors.iter_mut() {
                    device.reset_measures_minmax();
                }
            }
            CoproApiMessage::GetBleDevicesState { respond_to } => {
                respond_to.send(self.devices.clone()).ok();
            }
            CoproApiMessage::BleRemoveBonds => {
                self.request_remove_bonds();
            }
            CoproApiMessage::BleEnablePairingAdv { duration_s } => {
                self.request_enable_pairing_adv(duration_s);
            }
            CoproApiMessage::GetPairingAdvState { respond_to } => {
                respond_to
                    .send((self.pairing_adv_active, self.pairing_adv_duration_s))
                    .ok();
            }
            CoproApiMessage::GetFirmwareVersion { respond_to } => {
                respond_to
                    .send(self.firmware_version.as_ref().map(|v| v.as_semver_string()))
                    .ok();
            }
        }

        Ok(())
    }
}
