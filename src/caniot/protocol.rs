use core::fmt;
use std::fmt::Debug;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::{Deserialize, Serialize};

pub const CANIOT_ERROR_BASE: isize = 0x3A00;
pub const CANIOT_DEVICE_FILTER_ID: u32 = 1 << 2; /* bit 2 is 1 for response frames */
pub const CANIOT_DEVICE_FILTER_MASK: u32 = 1 << 2; /* bit 2 is 1 to filter frames by direction */

use embedded_can::{Frame as EmbeddedFrame, Id as EmbeddedId, StandardId};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("DeviceIdCreationError")]
    DeviceIdCreationError,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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

    pub fn new(did: u8) -> Result<Self, ProtocolError> {
        Self::try_from(did)
    }

    pub fn get_did(&self) -> u8 {
        (self.sub_id << 3) | self.class
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Frame<T>
where
    T: Serialize,
{
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

    pub fn to_can_frame<T>(&self) -> Result<T, ProtocolError>
    where
        T: EmbeddedFrame,
    {
        let data = self.data.to_data();
        Ok(EmbeddedFrame::new(self.get_id(), &data).unwrap())
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum RequestData {
    Telemetry {
        endpoint: Endpoint,
    },
    Command {
        endpoint: Endpoint,
        payload: Vec<u8>,
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
    pub fn to_data(&self) -> Vec<u8> {
        match self {
            RequestData::Telemetry { .. } => vec![],
            RequestData::Command { payload, .. } => payload.clone(),
            RequestData::AttributeRead { key } => key.to_le_bytes().to_vec(),
            RequestData::AttributeWrite { key, value } => {
                let mut data = key.to_le_bytes().to_vec();
                data.extend_from_slice(&value.to_le_bytes());
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
        payload: Vec<u8>,
    },
    Attribute {
        key: u16,
        value: u32,
    },
    Error {
        source: ErrorSource,
        error: Option<CaniotError>,
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

const ERROR_CODE_LEN: usize = 2;
const ARG_LEN: usize = 4;

pub fn parse_error_payload(
    endpoint: Option<Endpoint>,
    payload: &[u8],
) -> Result<ResponseData, ConversionError> {
    let len = payload.len();
    let error_code: Option<CaniotError> = if len >= 2 {
        CaniotError::from_i16(i16::from_le_bytes(payload[0..2].try_into().unwrap()))
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
        let data: Vec<u8> = frame.0.data().to_vec();

        if id.direction != Direction::Response {
            if id.action == Action::Write {
                return parse_error_payload(id.get_endpoint(), &data)
                    .map(|data| Frame { device_id, data })
                    .map_err(|e| e.into());
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

fn format_payload(payload: &Vec<u8>) -> String {
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

pub fn build_telemetry_request(device_id: DeviceId, endpoint: Endpoint) -> Request {
    Request {
        device_id,
        data: RequestData::Telemetry { endpoint },
    }
}
pub fn build_attribute_read_request(device_id: DeviceId, key: u16) -> Request {
    Request {
        device_id,
        data: RequestData::AttributeRead { key },
    }
}

pub fn build_attribute_write_request(device_id: DeviceId, key: u16, value: u32) -> Request {
    Request {
        device_id,
        data: RequestData::AttributeWrite { key, value },
    }
}

pub fn build_command_request(device_id: DeviceId, endpoint: Endpoint, payload: Vec<u8>) -> Request {
    Request {
        device_id,
        data: RequestData::Command { endpoint, payload },
    }
}

#[derive(Debug, Clone)]
pub struct ResponseMatch {
    is_reponse: bool,
    is_error: bool,
}

impl ResponseMatch {
    pub fn new(is_response: bool, is_error: bool) -> Self {
        Self {
            is_reponse: is_response,
            is_error: is_error,
        }
    }

    pub fn is_response(&self) -> bool {
        self.is_reponse
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn is_valid_response(&self) -> bool {
        self.is_reponse && !self.is_error
    }

    pub fn is_response_error(&self) -> bool {
        self.is_reponse && self.is_error
    }
}

fn response_match_any_telemetry_query(
    query_endpoint: Endpoint,
    response: &Response,
) -> ResponseMatch {
    let (is_response, is_error) = match response.data {
        ResponseData::Telemetry {
            endpoint: response_endpoint,
            ..
        } => (query_endpoint == response_endpoint, false),
        ResponseData::Error {
            source: ErrorSource::Telemetry(endpoint, _),
            ..
        } => (query_endpoint == endpoint, true),
        ResponseData::Error { .. } => (false, true),
        ResponseData::Attribute { .. } => (false, false),
    };

    ResponseMatch::new(is_response, is_error)
}

fn response_match_any_attribute_query(key: u16, response: &Response) -> ResponseMatch {
    let (is_response, is_error) = match response.data {
        ResponseData::Telemetry { .. } => (false, false),
        ResponseData::Attribute {
            key: response_key, ..
        } => (key == response_key, false),
        ResponseData::Error {
            source: ErrorSource::Attribute(err_key),
            ..
        } => (
            // unwrap_or(true) because if no key is present, we assume it is a response to any attribute query
            err_key.map(|err_key| key == err_key).unwrap_or(true),
            true,
        ),
        ResponseData::Error { .. } => (false, true),
    };

    ResponseMatch::new(is_response, is_error)
}

pub fn is_response_to(query: &Request, response: &Response) -> ResponseMatch {
    if query.device_id != DeviceId::BROADCAST && query.device_id != response.device_id {
        return ResponseMatch::new(false, false);
    }

    match query.data {
        RequestData::Command { endpoint, .. } | RequestData::Telemetry { endpoint } => {
            response_match_any_telemetry_query(endpoint, response)
        }
        RequestData::AttributeWrite { key, .. } | RequestData::AttributeRead { key } => {
            response_match_any_attribute_query(key, response)
        }
    }
}

///
/// Tests private functions
///

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_error_payload() {
        fn test_payload(
            endpoint: Option<Endpoint>,
            payload: &[u8],
            expected_source: ErrorSource,
            expected_error: Option<CaniotError>,
        ) {
            let resp = parse_error_payload(endpoint, payload).unwrap();
            assert_eq!(
                resp,
                ResponseData::Error {
                    source: expected_source,
                    error: expected_error
                }
            );
        }

        test_payload(
            Some(Endpoint::ApplicationDefault),
            &[],
            ErrorSource::Telemetry(Endpoint::ApplicationDefault, None),
            None,
        );

        test_payload(
            Some(Endpoint::ApplicationDefault),
            &[0x00, 0x00],
            ErrorSource::Telemetry(Endpoint::ApplicationDefault, None),
            Some(CaniotError::Ok),
        );

        test_payload(
            Some(Endpoint::ApplicationDefault),
            &[0x00, 0x3a, 0xFF, 0x00, 0x00, 0x00],
            ErrorSource::Telemetry(Endpoint::ApplicationDefault, Some(0xFF)),
            Some(CaniotError::Einval),
        );

        test_payload(None, &[0x00], ErrorSource::Attribute(None), None);

        test_payload(
            None,
            &[0x00, 0x3a, 0x00, 0x01, 0x00, 0x00],
            ErrorSource::Attribute(Some(0x0100)),
            Some(CaniotError::Einval),
        );
    }

    #[test]
    fn test_response_match_any_attribute_query() {
        // attribute
        let query = Request {
            device_id: DeviceId::new(1).unwrap(),
            data: RequestData::AttributeRead { key: 0x0100 },
        };

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Attribute {
                key: 0x0100,
                value: 0x12345678,
            },
        };
        assert!(is_response_to(&query, &response).is_valid_response());

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Error { 
                source: ErrorSource::Attribute(Some(0x0100)),
                error: None 
            },
        };
        assert!(is_response_to(&query, &response).is_response_error());

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Error { 
                source: ErrorSource::Attribute(None),
                error: None 
            },
        };
        assert!(is_response_to(&query, &response).is_response_error());

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Error { 
                source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
                error: None 
            },
        };
        let is_response = is_response_to(&query, &response);
        assert!(is_response.is_error() && !is_response.is_response());

        // telemetry
        let query = Request {
            device_id: DeviceId::new(1).unwrap(),
            data: RequestData::Telemetry { endpoint: Endpoint::Application2 },
        };

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Telemetry { endpoint: Endpoint::Application2, payload: vec![] },
        };
        assert!(is_response_to(&query, &response).is_valid_response());

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Telemetry { endpoint: Endpoint::Application1, payload: vec![] },
        };
        let m = is_response_to(&query, &response);
        assert!(!m.is_error() && !m.is_response());

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Error { 
                source: ErrorSource::Telemetry(Endpoint::Application2, None),
                error: None
            },
        };
        assert!(is_response_to(&query, &response).is_response_error());

        let response = Response {
            device_id: DeviceId::new(1).unwrap(),
            data: ResponseData::Error { 
                source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
                error: None
            },
        };
        let m = is_response_to(&query, &response);
        assert!(m.is_error() && !m.is_response());
    }
}
