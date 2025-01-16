use tokio::sync::oneshot;

use crate::controller::{device_filtering::DeviceFilter, DeviceAlert};

use super::{controller::CoproControllerStats, device::BleDevice};

pub enum CoproApiMessage {
    GetDevices {
        filter: DeviceFilter,
        respond_to: oneshot::Sender<Vec<BleDevice>>,
    },
    GetAlert {
        respond_to: oneshot::Sender<Option<DeviceAlert>>,
    },
    GetStats {
        respond_to: oneshot::Sender<CoproControllerStats>,
    },
    ResetDevicesMeasuresStats,
}
