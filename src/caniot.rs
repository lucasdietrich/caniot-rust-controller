use std::fmt::Debug;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub const CANIOT_ERROR_BASE: isize = 0x3A00;
pub const CANIOT_DEVICE_FILTER_ID: u32 = 1 << 2; /* bit 2 is 1 for response frames */
pub const CANIOT_DEVICE_FILTER_MASK: u32 = 1 << 2; /* bit 2 is 1 to filter frames by direction */

use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId};

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

#[derive(Clone, Copy)]
pub struct DeviceId {
    pub class: u8,
    pub sub_id: u8,
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

#[derive(Debug, Clone, Copy)]
pub struct AttributeKey {
    pub key: u16,
}

impl From<u16> for AttributeKey {
    fn from(key: u16) -> Self {
        AttributeKey { key }
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

#[derive(Debug)]
pub struct Frame {
    pub device_id: DeviceId,
    pub frame_type: FrameType,
}

#[derive(Debug)]
pub enum FrameType {
    Telemetry(TelemetryData),
    Attribute(AttributeData),
}

#[derive(Debug)]
pub struct TelemetryData {
    pub endpoint: Endpoint,
    pub payload: TelemetryContent,
}

#[derive(Debug)]
pub struct AttributeData {
    pub key: AttributeKey,
    pub payload: AttributeContent,
}

// pub struct TelemetryPayload {
//     pub data: [u8; 8],
// }

#[derive(Debug)]
pub enum TelemetryContent {
    Query,
    Response([u8; 8]),
    Command([u8; 8]),
    Error(ErrorData),
}

#[derive(Debug)]
pub enum AttributeContent {
    ReadRequest,
    WriteRequest(u32),
    Response(u32),
    Error(ErrorData),
}

#[derive(Debug)]
pub struct ErrorData {
    pub error: CaniotError,
}

impl From<u16> for Id {
    fn from(id: u16) -> Self {
        Id {
            device_id: DeviceId {
                class: ((id >> 3) & 0x7) as u8,
                sub_id: ((id >> 6) & 0x7) as u8,
            },
            action: Action::from_u8((id & 0x1) as u8).unwrap(),
            msg_type: Type::from_u8(((id >> 1) & 0x1) as u8).unwrap(),
            direction: Direction::from_u8(((id >> 2) & 0x1) as u8).unwrap(),
            endpoint: Endpoint::from_u8(((id >> 9) & 0x3) as u8).unwrap(),
        }
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

impl Frame {
    pub fn is_telemetry(&self) -> bool {
        matches!(self.frame_type, FrameType::Telemetry(_))
    }

    pub fn is_attribute(&self) -> bool {
        matches!(self.frame_type, FrameType::Attribute(_))
    }
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("TryFromSlice error: {0}")]
    TryFromSlice(#[from] std::array::TryFromSliceError),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
}

// This structure is used to encapsulate a Type which implements EmbeddedFrame
// This way we can implement TryFrom<EmbeddedFrameWrapper<E>> for Frame
// where E: EmbeddedFrame
pub struct EmbeddedFrameWrapper<T: EmbeddedFrame>(pub T);

impl<E> TryFrom<EmbeddedFrameWrapper<E>> for Frame
where
    E: EmbeddedFrame,
{
    type Error = ConversionError;

    fn try_from(frame: EmbeddedFrameWrapper<E>) -> Result<Self, Self::Error> {
        let id: Id = frame.0.id().try_into()?;
        let device_id = id.device_id;

        // The repetitive logic to construct Frame is extracted here
        fn make_frame(device_id: DeviceId, frame_type: FrameType) -> Frame {
            Frame {
                device_id,
                frame_type,
            }
        }

        let data: [u8; 8] = frame.0.data().try_into()?;

        Ok(match id.msg_type {
            Type::Telemetry => {
                let payload = match (id.direction, id.action) {
                    (Direction::Query, Action::Read) => TelemetryContent::Query,
                    (Direction::Query, Action::Write) => TelemetryContent::Command(data.clone()),
                    (Direction::Response, Action::Read) => TelemetryContent::Response(data.clone()),
                    (Direction::Response, Action::Write) => TelemetryContent::Error(ErrorData {
                        error: CaniotError::from_i16(i16::from_le_bytes([data[0], data[1]]))
                            .unwrap_or(CaniotError::Eunexpected),
                    }),
                };

                let telemetry_data = TelemetryData {
                    endpoint: id.endpoint,
                    payload,
                };
                make_frame(device_id, FrameType::Telemetry(telemetry_data))
            }

            Type::Attribute => {
                let mut attribute_key = AttributeKey {
                    key: data[0] as u16,
                };
                let payload = match (id.direction, id.action) {
                    (Direction::Query, Action::Read) => AttributeContent::ReadRequest,
                    (Direction::Query, Action::Write) => AttributeContent::WriteRequest(
                        u32::from_le_bytes(data[2..6].try_into().unwrap()),
                    ),
                    (Direction::Response, Action::Read) => AttributeContent::Response(
                        u32::from_le_bytes(data[2..6].try_into().unwrap()),
                    ),
                    (Direction::Response, Action::Write) => {
                        attribute_key.key = data[2] as u16;
                        AttributeContent::Error(ErrorData {
                            error: CaniotError::from_i16(i16::from_le_bytes([data[0], data[1]]))
                                .unwrap_or(CaniotError::Eunexpected),
                        })
                    }
                };

                let attribute_data = AttributeData {
                    key: attribute_key,
                    payload,
                };
                make_frame(device_id, FrameType::Attribute(attribute_data))
            }
        })
    }
}
