use num::FromPrimitive;

use crate::caniot::{Cd, HeatingMode, Payload, ProtocolError, Ty};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerCommand {
    pub modes: [HeatingMode; 4],
}

impl TryFrom<&Payload<Cd>> for HeatingControllerCommand {
    type Error = ProtocolError;

    fn try_from(payload: &Payload<Cd>) -> Result<Self, ProtocolError> {
        let payload = payload.as_ref();
        if payload.len() >= 2 {
            Ok(HeatingControllerCommand {
                modes: [
                    HeatingMode::from_u8(payload[0] & 0xf).unwrap(),
                    HeatingMode::from_u8((payload[0] & 0xf0) >> 4).unwrap(),
                    HeatingMode::from_u8(payload[1] & 0xf).unwrap(),
                    HeatingMode::from_u8((payload[1] & 0xf0) >> 4).unwrap(),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

impl Into<Payload<Cd>> for HeatingControllerCommand {
    fn into(self) -> Payload<Cd> {
        let mut payload = Vec::with_capacity(2);

        payload.push(self.modes[0] as u8 | (self.modes[1] as u8) << 4);
        payload.push(self.modes[2] as u8 | (self.modes[3] as u8) << 4);

        Payload::<Cd>::new(&payload).unwrap()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerTelemetry {
    pub modes: [HeatingMode; 4],
    pub power_status: bool,
}

impl TryFrom<&Payload<Ty>> for HeatingControllerTelemetry {
    type Error = ProtocolError;

    fn try_from(payload: &Payload<Ty>) -> Result<Self, ProtocolError> {
        let payload = payload.as_ref();
        if payload.len() >= 3 {
            Ok(HeatingControllerTelemetry {
                modes: [
                    HeatingMode::from_u8(payload[0] & 0xf).unwrap(),
                    HeatingMode::from_u8((payload[0] & 0xf0) >> 4).unwrap(),
                    HeatingMode::from_u8(payload[1] & 0xf).unwrap(),
                    HeatingMode::from_u8((payload[1] & 0xf0) >> 4).unwrap(),
                ],
                power_status: payload[2] & 0b0000_0001 != 0,
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

impl Into<Payload<Ty>> for HeatingControllerTelemetry {
    fn into(self) -> Payload<Ty> {
        let mut payload = Vec::with_capacity(3);

        payload.push(self.modes[0] as u8 | (self.modes[1] as u8) << 4);
        payload.push(self.modes[2] as u8 | (self.modes[3] as u8) << 4);
        payload.push(self.power_status as u8);

        Payload::new(&payload).unwrap()
    }
}

impl Into<Vec<u8>> for HeatingControllerTelemetry {
    fn into(self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(3);

        payload.push(self.modes[0] as u8 | (self.modes[1] as u8) << 4);
        payload.push(self.modes[2] as u8 | (self.modes[3] as u8) << 4);
        payload.push(self.power_status as u8);

        payload
    }
}
