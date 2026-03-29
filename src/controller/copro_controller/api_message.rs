use tokio::sync::oneshot;

use crate::controller::{filtering::SensorFilter, SensorAlert};

use super::{
    controller::{BleDevice, CoproControllerStats},
    sensor::BleSensor,
};

pub enum CoproApiMessage {
    GetDevices {
        filter: SensorFilter,
        respond_to: oneshot::Sender<Vec<BleSensor>>,
    },
    GetAlert {
        respond_to: oneshot::Sender<Option<SensorAlert>>,
    },
    GetStats {
        respond_to: oneshot::Sender<CoproControllerStats>,
    },
    ResetDevicesMeasuresStats,
    BleRemoveBonds,
    GetBleDevicesState {
        respond_to: oneshot::Sender<Vec<BleDevice>>,
    },
    BleEnablePairingAdv {
        duration_s: u32,
    },
    GetPairingAdvState {
        respond_to: oneshot::Sender<(bool, u32)>,
    },
}
