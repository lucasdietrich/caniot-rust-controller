use core::fmt;

use crate::caniot::{format_u8_list, Cd, Payload, ProtocolError, SysCtrl};

use super::{
    Action, ConversionError, DeviceId, Direction, Endpoint, Frame, Id, InnerFrameTrait, Type,
};
use chrono::Utc;
use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId, StandardId};
use serde::Serialize;
use socketcan::CanDataFrame;

pub type Request = Frame<RequestData>;

#[derive(Debug, Clone, Serialize)]
pub enum RequestData {
    Telemetry {
        endpoint: Endpoint,
    },
    Command {
        endpoint: Endpoint,
        payload: Payload<Cd>,
    },
    AttributeRead {
        key: u16,
    },
    AttributeWrite {
        key: u16,
        value: u32,
    },
}

impl InnerFrameTrait for RequestData {
    fn get_type(&self) -> Type {
        match self {
            RequestData::Telemetry { .. } => Type::Telemetry,
            RequestData::Command { .. } => Type::Telemetry,
            RequestData::AttributeRead { .. } => Type::Attribute,
            RequestData::AttributeWrite { .. } => Type::Attribute,
        }
    }

    fn get_endpoint(&self) -> Option<Endpoint> {
        match self {
            RequestData::Telemetry { endpoint } => Some(*endpoint),
            RequestData::Command { endpoint, .. } => Some(*endpoint),
            RequestData::AttributeRead { .. } => None,
            RequestData::AttributeWrite { .. } => None,
        }
    }

    fn get_key(&self) -> Option<u16> {
        match self {
            RequestData::Telemetry { .. } => None,
            RequestData::Command { .. } => None,
            RequestData::AttributeRead { key } => Some(*key),
            RequestData::AttributeWrite { key, .. } => Some(*key),
        }
    }
}

impl RequestData {
    /// Create a new board control request.
    ///
    /// Board control requests are used to control the board itself. (e.g. reset, watchdog, etc.)
    /// The endpoint is always set to `Endpoint::BoardControl`.
    ///
    /// # Arguments
    ///
    /// * `sys` - The system control data
    pub fn new_board_control_request(sys: SysCtrl) -> Self {
        RequestData::Command {
            endpoint: Endpoint::BoardControl,
            payload: sys.into(),
        }
    }

    fn get_can_payload(&self) -> Vec<u8> {
        match self {
            RequestData::Telemetry { .. } => vec![],
            RequestData::Command { payload, .. } => payload.as_ref().to_vec(),
            RequestData::AttributeRead { key } => key.to_le_bytes().to_vec(),
            RequestData::AttributeWrite { key, value } => {
                let mut data = key.to_le_bytes().to_vec();
                data.extend_from_slice(&value.to_le_bytes());
                data
            }
        }
    }

    pub fn into_broadcast(self) -> Request {
        Frame {
            device_id: DeviceId::BROADCAST,
            data: self,
            timestamp: Utc::now(),
        }
    }
}

impl Into<CanDataFrame> for &Frame<RequestData> {
    fn into(self) -> CanDataFrame {
        CanDataFrame::new(self.get_can_id(), self.get_can_payload().as_ref()).unwrap()
    }
}

impl Into<CanDataFrame> for Frame<RequestData> {
    fn into(self) -> CanDataFrame {
        (&self).into()
    }
}

impl TryFrom<CanDataFrame> for Frame<RequestData> {
    type Error = ConversionError;

    fn try_from(value: CanDataFrame) -> Result<Self, Self::Error> {
        let id = Id::try_from(value.id())?;
        let payload = value.data();

        let data = if id.direction == Direction::Query {
            match (id.msg_type, id.action) {
                (Type::Telemetry, Action::Read) => RequestData::Telemetry {
                    endpoint: id.endpoint,
                },
                (Type::Telemetry, Action::Write) => RequestData::Command {
                    endpoint: id.endpoint,
                    payload: Payload::new_unchecked(payload),
                },
                (Type::Attribute, Action::Read) => RequestData::AttributeRead {
                    key: u16::from_le_bytes(payload[0..2].try_into().expect("Invalid attr key")),
                },
                (Type::Attribute, Action::Write) => RequestData::AttributeWrite {
                    key: u16::from_le_bytes(payload[0..2].try_into().expect("Invalid attr key")),
                    value: u32::from_le_bytes(
                        payload[2..6].try_into().expect("Invalid attr value"),
                    ),
                },
            }
        } else {
            return Err(ConversionError::NotValidRequest);
        };

        Ok(Frame {
            device_id: id.device_id,
            data,
            timestamp: Utc::now(),
        })
    }
}

impl Request {
    pub fn get_can_id(&self) -> EmbeddedId {
        let id = match &self.data {
            RequestData::Telemetry { endpoint, .. } => Id {
                device_id: self.device_id,
                direction: Direction::Query,
                msg_type: Type::Telemetry,
                action: Action::Read,
                endpoint: *endpoint,
            },
            RequestData::Command { endpoint, .. } => Id {
                device_id: self.device_id,
                direction: Direction::Query,
                msg_type: Type::Telemetry,
                action: Action::Write,
                endpoint: *endpoint,
            },
            RequestData::AttributeRead { .. } => Id {
                device_id: self.device_id,
                direction: Direction::Query,
                msg_type: Type::Attribute,
                action: Action::Read,
                endpoint: Endpoint::ApplicationDefault,
            },
            RequestData::AttributeWrite { .. } => Id {
                device_id: self.device_id,
                direction: Direction::Query,
                msg_type: Type::Attribute,
                action: Action::Write,
                endpoint: Endpoint::ApplicationDefault,
            },
        };
        let can_id = StandardId::new(id.to_u16()).unwrap();
        EmbeddedId::Standard(can_id)
    }

    pub fn get_can_payload(&self) -> Vec<u8> {
        self.data.get_can_payload()
    }

    pub fn to_can_frame<T>(&self) -> Result<T, ProtocolError>
    where
        T: EmbeddedFrame,
    {
        let data = self.data.get_can_payload();
        Ok(EmbeddedFrame::new(self.get_can_id(), &data).unwrap())
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            RequestData::Telemetry { endpoint } => {
                write!(f, "Telemetry Request {}: {}", self.device_id, endpoint)
            }
            RequestData::Command { endpoint, payload } => {
                write!(
                    f,
                    "Command Request {}: {} / {}",
                    self.device_id,
                    endpoint,
                    format_u8_list(payload)
                )
            }
            RequestData::AttributeRead { key } => {
                write!(f, "Attribute Read Request {}: key {}", self.device_id, key)
            }
            RequestData::AttributeWrite { key, value } => {
                write!(
                    f,
                    "Attribute Write Request {}: key {} write {}",
                    self.device_id, key, value
                )
            }
        }
    }
}
