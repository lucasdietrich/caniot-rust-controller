use core::fmt;
use std::fmt::Debug;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::Serialize;

pub const CANIOT_DEVICE_FILTER_ID: u32 = 1 << 2; /* bit 2 is 1 for response frames */
pub const CANIOT_DEVICE_FILTER_MASK: u32 = 1 << 2; /* bit 2 is 1 to filter frames by direction */

use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId, StandardId};

use socketcan::CanDataFrame;
use thiserror::Error;

use super::{CommandPL, DeviceId, ErrorCode, Payload, ProtocolError, TelemetryPL};

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
    device_id: DeviceId,
    direction: Direction,
    msg_type: Type,
    action: Action,
    endpoint: Endpoint,
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

pub type Request = Frame<RequestData>;
pub type Response = Frame<ResponseData>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Frame<T>
where
    T: Serialize,
{
    pub device_id: DeviceId,
    pub data: T,
}

impl<T> Frame<T>
where
    T: Serialize,
{
    pub fn new(device_id: DeviceId, data: T) -> Self {
        Self { device_id, data }
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

#[derive(Debug, Clone, Serialize)]
pub enum RequestData {
    Telemetry {
        endpoint: Endpoint,
    },
    Command {
        endpoint: Endpoint,
        payload: Payload<CommandPL>,
    },
    AttributeRead {
        key: u16,
    },
    AttributeWrite {
        key: u16,
        value: u32,
    },
}

impl RequestData {
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ErrorSource {
    Telemetry(Endpoint, Option<u32>),
    Attribute(Option<u16>),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ResponseData {
    Telemetry {
        endpoint: Endpoint,
        payload: Payload<TelemetryPL>,
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

#[derive(Debug)]
pub struct ErrorData {
    pub error: ErrorCode,
}

impl Frame<ResponseData> {
    pub fn is_telemetry(&self) -> bool {
        matches!(self.data, ResponseData::Telemetry { .. })
    }

    pub fn is_attribute(&self) -> bool {
        matches!(self.data, ResponseData::Attribute { .. })
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
        })
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

fn format_payload(payload: impl AsRef<[u8]>) -> String {
    payload
        .as_ref()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<String>>()
        .join(" ")
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
                    format_payload(payload)
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
                    format_payload(payload)
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
