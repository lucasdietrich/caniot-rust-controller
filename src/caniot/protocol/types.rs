use core::fmt;

use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::Serialize;
use thiserror::Error;

use crate::caniot::DeviceId;

use embedded_can::{Id as EmbeddedId, StandardId};

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
pub enum Type {
    Telemetry = 0,
    Attribute = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
pub enum Action {
    Write = 0,
    Read = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
pub enum Direction {
    Query = 0,
    Response = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize)]
pub enum Endpoint {
    ApplicationDefault = 0,
    Application1 = 1,
    Application2 = 2,
    BoardControl = 3,
}

impl TryFrom<i32> for Endpoint {
    type Error = ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Endpoint::ApplicationDefault),
            1 => Ok(Endpoint::Application1),
            2 => Ok(Endpoint::Application2),
            3 => Ok(Endpoint::BoardControl),
            _ => Err(ConversionError::NotValidEndpoint),
        }
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Endpoint::ApplicationDefault => write!(f, "ep-0"),
            Endpoint::Application1 => write!(f, "ep-1"),
            Endpoint::Application2 => write!(f, "ep-2"),
            Endpoint::BoardControl => write!(f, "ep-c"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Id {
    pub(crate) device_id: DeviceId,
    pub(crate) direction: Direction,
    pub(crate) msg_type: Type,
    pub(crate) action: Action,
    pub(crate) endpoint: Endpoint,
}

impl From<u16> for Id {
    fn from(id: u16) -> Self {
        Id {
            device_id: DeviceId::try_from(((id >> 3) & 0x3f) as u8).unwrap(),
            action: Action::from_u8((id & 0x1) as u8).unwrap(),
            msg_type: Type::from_u8(((id >> 1) & 0x1) as u8).unwrap(),
            direction: Direction::from_u8(((id >> 2) & 0x1) as u8).unwrap(),
            endpoint: Endpoint::from_u8(((id >> 9) & 0x3) as u8).unwrap(),
        }
    }
}

impl Id {
    // Direct conversion functions instead of Into traits
    pub fn to_u16(self) -> u16 {
        let mut id: u16 = 0;
        id |= (self.device_id.class as u16) << 3;
        id |= (self.device_id.sub_id as u16) << 6;
        id |= self.action as u16;
        id |= (self.msg_type as u16) << 1;
        id |= (self.direction as u16) << 2;
        id |= (self.endpoint as u16) << 9;
        id
    }

    #[allow(dead_code)]
    pub fn to_embedded_id(self) -> EmbeddedId {
        let std_can_id = StandardId::new(self.to_u16()).unwrap();
        EmbeddedId::Standard(std_can_id)
    }

    /// Returns the endpoint if the message is a telemetry message
    /// Returns None if the message is not a attribute message
    pub fn get_endpoint(&self) -> Option<Endpoint> {
        if self.msg_type == Type::Telemetry {
            Some(self.endpoint)
        } else {
            None
        }
    }
}

impl TryFrom<EmbeddedId> for Id {
    type Error = ConversionError;

    fn try_from(value: EmbeddedId) -> Result<Self, Self::Error> {
        match value {
            EmbeddedId::Standard(id) => Ok(id.as_raw().into()),
            EmbeddedId::Extended(_) => Err(ConversionError::NotValidId),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Not a valid caniot response frame")]
    NotValidResponse,
    #[error("Not a valid caniot request frame")]
    NotValidRequest,
    #[error("Not a valid caniot id")]
    NotValidId,
    #[error("Not a valid caniot endpoint")]
    NotValidEndpoint,
}
