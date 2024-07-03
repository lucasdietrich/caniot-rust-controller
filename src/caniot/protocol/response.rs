use core::fmt;

use crate::caniot::{format_u8_list, ErrorCode, Payload, ProtocolError, Ty};

use super::{Action, ConversionError, Direction, Endpoint, Frame, Id, InnerFrameTrait, Type};
use chrono::Utc;
use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId, StandardId};
use num::FromPrimitive;
use serde::Serialize;
use socketcan::CanDataFrame;

pub type Response = Frame<ResponseData>;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ErrorSource {
    Telemetry(Endpoint, Option<u32>),
    Attribute(Option<u16>),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ResponseData {
    Telemetry {
        endpoint: Endpoint,
        payload: Payload<Ty>,
    },
    Attribute {
        key: u16,
        value: u32,
    },
    Error {
        source: ErrorSource,
        error: Option<ErrorCode>,
    },
}

impl ResponseData {
    fn to_data(&self) -> Vec<u8> {
        match self {
            ResponseData::Telemetry { payload, .. } => payload.as_ref().to_vec(),
            ResponseData::Attribute { key, value } => {
                let mut data = key.to_le_bytes().to_vec();
                data.extend_from_slice(&value.to_le_bytes());
                data
            }
            ResponseData::Error { source, error } => {
                let mut data = vec![0; 4];
                if let Some(err) = error {
                    data[0..4].copy_from_slice(&err.value().to_le_bytes());
                } else if let ErrorSource::Attribute(arg) = source {
                    if let Some(arg) = arg {
                        data[4..8].copy_from_slice(&arg.to_le_bytes());
                    }
                }
                data
            }
        }
    }
}

impl InnerFrameTrait for ResponseData {
    fn get_type(&self) -> Type {
        match self {
            ResponseData::Telemetry { .. } => Type::Telemetry,
            ResponseData::Attribute { .. } => Type::Attribute,
            ResponseData::Error { source, .. } => match source {
                ErrorSource::Telemetry(_, _) => Type::Telemetry,
                ErrorSource::Attribute(_) => Type::Attribute,
            },
        }
    }

    fn get_endpoint(&self) -> Option<Endpoint> {
        match self {
            ResponseData::Telemetry { endpoint, .. } => Some(*endpoint),
            ResponseData::Attribute { .. } => None,
            ResponseData::Error { source, .. } => match source {
                ErrorSource::Telemetry(endpoint, _) => Some(*endpoint),
                ErrorSource::Attribute(_) => None,
            },
        }
    }

    fn get_key(&self) -> Option<u16> {
        match self {
            ResponseData::Telemetry { .. } => None,
            ResponseData::Attribute { key, .. } => Some(*key),
            ResponseData::Error { source, .. } => match source {
                ErrorSource::Telemetry(_, _) => None,
                ErrorSource::Attribute(arg) => *arg,
            },
        }
    }
}

#[derive(Debug)]
pub struct ErrorData {
    pub error: ErrorCode,
}

const ERROR_CODE_LEN: usize = 4;
const ARG_LEN: usize = 4;

pub fn parse_error_payload(
    endpoint: Option<Endpoint>,
    payload: &[u8],
) -> Result<ResponseData, ConversionError> {
    let len = payload.len();
    let error_code: Option<ErrorCode> = if len >= ERROR_CODE_LEN {
        ErrorCode::from_i32(i32::from_le_bytes(
            payload[0..ERROR_CODE_LEN]
                .try_into()
                .expect("Invalid error code"),
        ))
    } else {
        None
    };

    let arg = payload
        .get(ERROR_CODE_LEN..ERROR_CODE_LEN + ARG_LEN)
        .and_then(|bytes| bytes.try_into().ok())
        .map(u32::from_le_bytes);

    let source = match endpoint {
        Some(ep) => ErrorSource::Telemetry(ep, arg),
        None => ErrorSource::Attribute(arg.map(|x| x as u16)),
    };

    Ok(ResponseData::Error {
        source,
        error: error_code,
    })
}

impl TryFrom<CanDataFrame> for Frame<ResponseData> {
    type Error = ConversionError;

    fn try_from(frame: CanDataFrame) -> Result<Self, Self::Error> {
        let id = Id::try_from(frame.id())?;
        let payload = frame.data();

        let data = if id.direction == Direction::Response {
            if id.action == Action::Read {
                match id.msg_type {
                    Type::Telemetry => ResponseData::Telemetry {
                        endpoint: id.endpoint,
                        payload: Payload::new_unchecked(payload),
                    },
                    Type::Attribute => ResponseData::Attribute {
                        key: u16::from_le_bytes(
                            payload[0..2].try_into().expect("Invalid attr key"),
                        ),
                        value: u32::from_le_bytes(
                            payload[2..6].try_into().expect("Invalid attr value"),
                        ),
                    },
                }
            } else {
                parse_error_payload(id.get_endpoint(), &payload)?
            }
        } else {
            return Err(ConversionError::NotValidResponse);
        };

        Ok(Frame {
            device_id: id.device_id,
            data,
            timestamp: Utc::now(),
        })
    }
}

impl Into<CanDataFrame> for Frame<ResponseData> {
    fn into(self) -> CanDataFrame {
        (&self).into()
    }
}

impl Into<CanDataFrame> for &Frame<ResponseData> {
    fn into(self) -> CanDataFrame {
        CanDataFrame::new(self.get_can_id(), self.get_can_payload().as_ref()).unwrap()
    }
}

impl Response {
    pub fn get_can_id(&self) -> EmbeddedId {
        let id = match &self.data {
            ResponseData::Telemetry { endpoint, .. } => Id {
                device_id: self.device_id,
                direction: Direction::Response,
                msg_type: Type::Telemetry,
                action: Action::Read,
                endpoint: *endpoint,
            },
            ResponseData::Attribute { .. } => Id {
                device_id: self.device_id,
                direction: Direction::Response,
                msg_type: Type::Attribute,
                action: Action::Read,
                endpoint: Endpoint::ApplicationDefault,
            },
            ResponseData::Error { source, .. } => match source {
                ErrorSource::Telemetry(endpoint, _) => Id {
                    device_id: self.device_id,
                    direction: Direction::Response,
                    msg_type: Type::Telemetry,
                    action: Action::Write,
                    endpoint: *endpoint,
                },
                ErrorSource::Attribute(_) => Id {
                    device_id: self.device_id,
                    direction: Direction::Response,
                    msg_type: Type::Attribute,
                    action: Action::Write,
                    endpoint: Endpoint::ApplicationDefault,
                },
            },
        };
        let can_id = StandardId::new(id.to_u16()).unwrap();
        EmbeddedId::Standard(can_id)
    }

    pub fn get_can_payload(&self) -> Vec<u8> {
        self.data.to_data()
    }

    pub fn to_can_frame<T>(&self) -> Result<T, ProtocolError>
    where
        T: EmbeddedFrame,
    {
        let data = self.data.to_data();
        Ok(EmbeddedFrame::new(self.get_can_id(), &data).unwrap())
    }

    pub fn is_telemetry(&self) -> bool {
        matches!(self.data, ResponseData::Telemetry { .. })
    }

    pub fn is_attribute(&self) -> bool {
        matches!(self.data, ResponseData::Attribute { .. })
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ResponseData::Telemetry { endpoint, payload } => {
                write!(
                    f,
                    "Telemetry Response {}: {} / {}",
                    self.device_id,
                    endpoint,
                    format_u8_list(payload)
                )
            }
            ResponseData::Attribute { key, value } => {
                // Modify as per the actual representation of the key and value
                write!(f, "Attribute: key {} = value {}", key, value)
            }
            ResponseData::Error { error, source } => {
                write!(f, "Error: {:?} Source {:?}", error, source)
            }
        }
    }
}
