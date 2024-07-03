use serde::Serialize;

use crate::caniot::{Payload, ProtocolError, Ty};

use super::{
    class0::{self, Class0},
    class1::{self, Class1},
    traits::{Class, ClassTelemetryTrait},
};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum BoardClassTelemetry {
    Class0(class0::Telemetry),
    Class1(class1::Telemetry),
}

impl BoardClassTelemetry {
    pub fn class_id(&self) -> u8 {
        match self {
            BoardClassTelemetry::Class0(_) => 0,
            BoardClassTelemetry::Class1(_) => 1,
        }
    }

    pub fn as_class0(&self) -> Option<&class0::Telemetry> {
        match self {
            BoardClassTelemetry::Class0(telemetry) => Some(telemetry),
            _ => None,
        }
    }

    pub fn as_class1(&self) -> Option<&class1::Telemetry> {
        match self {
            BoardClassTelemetry::Class1(telemetry) => Some(telemetry),
            _ => None,
        }
    }

    pub fn get_board_temperature(&self) -> Option<f32> {
        match self {
            BoardClassTelemetry::Class0(telemetry) => telemetry.get_board_temperature(),
            BoardClassTelemetry::Class1(telemetry) => telemetry.get_board_temperature(),
        }
    }

    pub fn get_outside_temperature(&self) -> Option<f32> {
        match self {
            BoardClassTelemetry::Class0(telemetry) => telemetry.get_outside_temperature(),
            BoardClassTelemetry::Class1(telemetry) => telemetry.get_outside_temperature(),
        }
    }
}

impl TryFrom<&Payload<Ty>> for BoardClassTelemetry {
    type Error = ProtocolError;

    fn try_from(_value: &Payload<Ty>) -> Result<Self, Self::Error> {
        Err(ProtocolError::UnsupportedClass) // cannot infer the class from the payload
    }
}

impl Into<Payload<Ty>> for BoardClassTelemetry {
    fn into(self) -> Payload<Ty> {
        match self {
            BoardClassTelemetry::Class0(class0_telemetry) => class0_telemetry.into(),
            BoardClassTelemetry::Class1(class1_telemetry) => class1_telemetry.into(),
        }
    }
}

#[allow(dead_code)]
pub fn boardlc_parse_telemetry<C: Class>(
    payload: &Payload<Ty>,
) -> Result<C::Telemetry, <<C as Class>::Telemetry as TryFrom<&Payload<Ty>>>::Error> {
    C::Telemetry::try_from(payload)
}

pub fn boardlc_parse_telemetry_as_class(
    class: u8,
    payload: &Payload<Ty>,
) -> Result<BoardClassTelemetry, ProtocolError> {
    match class {
        0 => Ok(BoardClassTelemetry::Class0(
            <Class0 as Class>::Telemetry::try_from(payload)?,
        )),
        1 => Ok(BoardClassTelemetry::Class1(
            <Class1 as Class>::Telemetry::try_from(payload)?,
        )),
        _ => Err(ProtocolError::UnsupportedClass),
    }
}
