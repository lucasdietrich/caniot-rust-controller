use core::fmt;
use std::fmt::Debug;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub const CANIOT_ERROR_BASE: isize = 0x3A00;
pub const CANIOT_DEVICE_FILTER_ID: u32 = 1 << 2; /* bit 2 is 1 for response frames */
pub const CANIOT_DEVICE_FILTER_MASK: u32 = 1 << 2; /* bit 2 is 1 to filter frames by direction */

use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId, StandardId};

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
pub enum CaniotError {
    Ok = 0x0000,
    Einval = CANIOT_ERROR_BASE, // Invalid argument
    Enproc,                     // UNPROCESSABLE
    Ecmd,                       // COMMAND
    Ekey,                       // KEY (read/write-attribute)
    Etimeout,                   // TIMEOUT
    Eagain,                     // BUSY / EAGAIN
    Efmt,                       // FORMAT
    Ehandlerc,                  // UNDEFINED COMMAND HANDLER
    Ehandlert,                  // UNDEFINED TELEMETRY HANDLER
    Etelemetry,                 // TELEMETRY
    Eunexpected,                // Unexpected frame
    Eep,                        // ENDPOINT
    Ecmdep,                     // ILLEGAL COMMAND, BROADCAST TO ALL ENDPOINTS
    Euninit,                    // NOT INITIALIZED
    Edriver,                    // DRIVER
    Eapi,                       // API
    Ekeysection,                // Unknown attributes section
    Ekeyattr,                   // Unknown attribute
    Ekeypart,                   // Unknown attribute part
    Enoattr,                    // No attribute
    Eclsattr,                   // Class attribute not accessible for current device
    Ereadonly,
    Enull,
    EnullDrv,
    EnullApi,
    EnullId,
    EnullDev,
    EnullCfg,
    EnullCtrl,
    EnullCtrlCb,
    Eroattr,    // READ-ONLY ATTRIBUTE
    Ereadattr,  // QUERY READ ATTR
    Ewriteattr, // QUERY WRITE ATTR
    Eenocb,     // no event handler
    Eecb,       // ECCB
    Epqalloc,   // PENDING QUERY ALLOCATION
    Enopq,      // NO PENDING QUERY
    Enohandle,  // NO HANDLER
    Edevice,    // DEVICE
    Eframe,     // FRAME, not a valid caniot frame
    Emlfrm,     // MALFORMED FRAME
    Eclass,     // INVALID CLASS
    Ecfg,       // INVALID CONFIGURATION
    Ehyst,      // Invalid hysteresis structure
    Enotsup,    // NOT SUPPORTED
    Enimpl,     // NOT IMPLEMENTED
}

impl CaniotError {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceId {
    pub class: u8,
    pub sub_id: u8,
}

impl From<u8> for DeviceId {
    fn from(id: u8) -> Self {
        DeviceId {
            class: id & 0x7,
            sub_id: (id >> 3) & 0x7,
        }
    }
}

impl DeviceId {
    pub fn get_did(&self) -> u8 {
        (self.sub_id << 3) | self.class
    }

    pub fn is_broadcast(&self) -> bool {
        self.get_did() == 0x7F
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}: {},{})", self.get_did(), self.class, self.sub_id)
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
pub enum Endpoint {
    ApplicationDefault = 0,
    Application1 = 1,
    Application2 = 2,
    BoardControl = 3,
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
            device_id: DeviceId::from((id >> 3) as u8),
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
}

impl TryFrom<EmbeddedId> for Id {
    type Error = std::io::Error;

    fn try_from(value: EmbeddedId) -> Result<Self, Self::Error> {
        match value {
            EmbeddedId::Standard(id) => Ok(id.as_raw().into()),
            EmbeddedId::Extended(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Extended ID not supported",
            )),
        }
    }
}

pub type Request = Frame<RequestData>;
pub type Response = Frame<ResponseData>;

#[derive(Debug)]
pub struct Frame<T> {
    pub device_id: DeviceId,
    pub data: T,
}

impl Request {
    pub fn get_id(&self) -> EmbeddedId {
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

    pub fn to_can_frame<T>(&self) -> T
    where
        T: EmbeddedFrame,
    {
        // TODO implement this
        EmbeddedFrame::new(self.get_id(), &[0u8; 8]).unwrap()
    }
}

#[derive(Debug)]
pub enum RequestData {
    Telemetry {
        endpoint: Endpoint,
    },
    Command {
        endpoint: Endpoint,
        payload: [u8; 8],
    },
    AttributeRead {
        key: u16,
    },
    AttributeWrite {
        key: u16,
        value: u32,
    },
}

#[derive(Debug)]
pub enum ResponseData {
    Telemetry {
        endpoint: Endpoint,
        payload: [u8; 8],
    },
    Attribute {
        key: u16,
        value: u32,
    },
    Error {
        error: CaniotError,
    },
}

#[derive(Debug)]
pub struct ErrorData {
    pub error: CaniotError,
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
    #[error("TryFromSlice error: {0}")]
    TryFromSlice(#[from] std::array::TryFromSliceError),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Not a CANIOT response frame")]
    NotCaniotResponse,
}

// This structure is used to encapsulate a Type which implements EmbeddedFrame
// This way we can implement TryFrom<EmbeddedFrameWrapper<E>> for Frame
// where E: EmbeddedFrame
pub struct EmbeddedFrameWrapper<T: EmbeddedFrame>(pub T);

impl<E> TryFrom<EmbeddedFrameWrapper<E>> for Frame<ResponseData>
where
    E: EmbeddedFrame,
{
    type Error = ConversionError;

    fn try_from(frame: EmbeddedFrameWrapper<E>) -> Result<Self, Self::Error> {
        let id: Id = frame.0.id().try_into()?;
        let device_id = id.device_id;
        let data: [u8; 8] = frame.0.data().try_into()?;

        if id.direction != Direction::Response {
            if id.action == Action::Write {
                return Ok(Frame {
                    device_id,
                    data: ResponseData::Error {
                        error: CaniotError::from_i16(i16::from_le_bytes(
                            data[0..2].try_into().unwrap(),
                        ))
                        .unwrap_or(CaniotError::Eunexpected),
                    },
                });
            } else {
                return Err(ConversionError::NotCaniotResponse);
            }
        }

        match id.msg_type {
            Type::Telemetry => Ok(Frame {
                device_id,
                data: ResponseData::Telemetry {
                    endpoint: id.endpoint,
                    payload: data,
                },
            }),
            Type::Attribute => Ok(Frame {
                device_id,
                data: ResponseData::Attribute {
                    key: u16::from_le_bytes(data[0..2].try_into().unwrap()),
                    value: u32::from_le_bytes(data[2..6].try_into().unwrap()),
                },
            }),
        }
    }
}

fn format_payload(payload: &[u8; 8]) -> String {
    payload
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
            ResponseData::Error { error } => {
                write!(f, "Error: {:?}", error)
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
