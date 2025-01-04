use tokio::sync::oneshot;

use crate::controller::DeviceAlert;

use super::{controller::CoproControllerStats, device::BleDevice};

pub enum CoproApiMessage {
    GetDevices {
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
