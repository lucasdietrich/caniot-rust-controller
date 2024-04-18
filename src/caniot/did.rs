use core::fmt;
use std::fmt::Debug;

use serde::Serialize;

use super::ProtocolError;

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId {
    pub class: u8,
    pub sub_id: u8,
}

impl TryFrom<u8> for DeviceId {
    type Error = ProtocolError;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        if id > 0x3f {
            return Err(ProtocolError::DeviceIdCreationError);
        } else {
            Ok(DeviceId {
                class: id & 0x7,
                sub_id: (id >> 3) & 0x7,
            })
        }
    }
}

impl DeviceId {
    pub const BROADCAST: DeviceId = DeviceId {
        class: 0x7,
        sub_id: 0x7,
    };

    pub fn new(class: u8, sub_id: u8) -> Result<Self, ProtocolError> {
        if class > 0x7 || sub_id > 0x7 {
            Err(ProtocolError::DeviceIdCreationError)
        } else {
            Ok(DeviceId { class, sub_id })
        }
    }

    pub fn from_u8(did: u8) -> Result<Self, ProtocolError> {
        Self::try_from(did)
    }

    pub fn to_u8(&self) -> u8 {
        (self.sub_id << 3) | self.class
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {},{})", self.to_u8(), self.class, self.sub_id)
    }
}
