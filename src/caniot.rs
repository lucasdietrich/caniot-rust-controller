use std::fmt::Debug;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy)]
pub struct DeviceId {
    class: u8,
    sub_id: u8,
}

impl DeviceId {
    pub fn get_did(&self) -> u8 {
        (self.sub_id << 3) | self.class
    }
}

impl Debug for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}: {})", self.class, self.sub_id, self.get_did())
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Type {
    Command = 0,
    Telemetry = 1,
    WriteAttribute = 2,
    ReadAttribute = 3,
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Direction {
    Query = 0,
    Response = 1,
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum Endpoint {
    ApplicationDefault = 0,
    Application1 = 1,
    Application2 = 2,
    BoardControl = 3,
}

#[derive(Clone, Copy)]
pub struct Id {
    device_id: DeviceId,
    endpoint: Endpoint,
    direction: Direction,
    msg_type: Type,
}

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        Id {
            device_id: DeviceId {
                class: ((id >> 3) & 0x7) as u8,
                sub_id: ((id >> 6) & 0x7) as u8,
            },
            msg_type: Type::from_u8((id & 0x3) as u8).unwrap(),
            direction: Direction::from_u8(((id >> 2) & 0x1) as u8).unwrap(),
            endpoint: Endpoint::from_u8(((id >> 9) & 0x3) as u8).unwrap(),
        }
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Caniot Id {:?} {:?} {:?} {:?}({})",
            self.device_id, self.direction, self.msg_type, self.endpoint, self.endpoint as u8
        )
    }
}