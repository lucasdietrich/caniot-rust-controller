use tokio::sync::oneshot;

use super::devices::BleDevice;

pub enum CoproApiMessage {
    GetDevices {
        respond_to: oneshot::Sender<Vec<BleDevice>>,
    },
}
