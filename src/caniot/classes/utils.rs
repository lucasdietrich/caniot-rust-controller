use crate::caniot::{BlcClassTelemetry, ProtocolError};

use super::{class0::Class0, class1::Class1, traits::Class};

pub fn blc_parse_telemetry<'a, C: Class<'a>>(
    payload: &'a [u8],
) -> Result<C::Telemetry, <<C as Class<'a>>::Telemetry as TryFrom<&'a [u8]>>::Error> {
    C::Telemetry::try_from(payload)
}

pub fn blc_parse_telemetry_as_class<'a>(
    class: u8,
    payload: &'a [u8],
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
