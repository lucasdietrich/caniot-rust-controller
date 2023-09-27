use crate::caniot::DeviceId;

struct Device {
    pub device_id: DeviceId,
}

struct Controller {
    pub devices: [Device; 63],
}

// impl Controller {
//     pub fn new() -> Self {
//         Controller {
//             devices: Default::default(),
//         }
//     }
// }
