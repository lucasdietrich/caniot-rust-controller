use serde::Serialize;

use crate::caniot::{Payload, ProtocolError, TelemetryPL};

use super::{
    class0::{self, Class0},
    class1::{self, Class1},
    traits::Class,
};

#[derive(Debug, Clone, Copy, Serialize)]
pub enum BlcClassTelemetry {
    Class0(class0::Telemetry),
    Class1(class1::Telemetry),
}

impl BlcClassTelemetry {
    pub fn class_id(&self) -> u8 {
        match self {
            BlcClassTelemetry::Class0(_) => 0,
            BlcClassTelemetry::Class1(_) => 1,
        }
    }

    pub fn as_class0(&self) -> Option<&class0::Telemetry> {
        match self {
            BlcClassTelemetry::Class0(telemetry) => Some(telemetry),
            _ => None,
        }
    }

    pub fn as_class1(&self) -> Option<&class1::Telemetry> {
        match self {
            BlcClassTelemetry::Class1(telemetry) => Some(telemetry),
            _ => None,
        }
    }

    pub fn get_board_temperature(&self) -> Option<f32> {
        match self {
            BlcClassTelemetry::Class0(telemetry) => telemetry.temp_in.to_celsius(),
            BlcClassTelemetry::Class1(telemetry) => telemetry.temp_in.to_celsius(),
        }
    }
}

impl Into<Payload<TelemetryPL>> for BlcClassTelemetry {
    fn into(self) -> Payload<TelemetryPL> {
        match self {
            BlcClassTelemetry::Class0(class0_telemetry) => class0_telemetry.into(),
            BlcClassTelemetry::Class1(class1_telemetry) => class1_telemetry.into(),
        }
    }
}

pub fn blc_parse_telemetry<C: Class>(
    payload: &Payload<TelemetryPL>,
) -> Result<C::Telemetry, <<C as Class>::Telemetry as TryFrom<&Payload<TelemetryPL>>>::Error> {
    C::Telemetry::try_from(payload)
}

pub fn blc_parse_telemetry_as_class(
    class: u8,
    payload: &Payload<TelemetryPL>,
) -> Result<BlcClassTelemetry, ProtocolError> {
    match class {
        0 => Ok(BlcClassTelemetry::Class0(
            <Class0 as Class>::Telemetry::try_from(payload)?,
        )),
        1 => Ok(BlcClassTelemetry::Class1(
            <Class1 as Class>::Telemetry::try_from(payload)?,
        )),
        _ => Err(ProtocolError::UnsupportedClass),
    }
}
