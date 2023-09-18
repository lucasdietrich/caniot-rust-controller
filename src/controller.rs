use crate::caniot::DeviceId;

struct Device {
    pub device_id: DeviceId,
}

struct Controller {
    pub devices: [Device; 63],
}
