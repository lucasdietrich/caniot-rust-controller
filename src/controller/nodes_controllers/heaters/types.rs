use num::FromPrimitive;

use crate::caniot::{CommandPL, HeatingMode, Payload, ProtocolError, TelemetryPL};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerCommand {
    pub modes: [HeatingMode; 4],
}

impl TryFrom<&Payload<CommandPL>> for HeatingControllerCommand {
    type Error = ProtocolError;

    fn try_from(payload: &Payload<CommandPL>) -> Result<Self, ProtocolError> {
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

impl Into<Payload<CommandPL>> for HeatingControllerCommand {
    fn into(self) -> Payload<CommandPL> {
        let mut payload = Vec::with_capacity(2);

        payload.push(self.modes[0] as u8 | (self.modes[1] as u8) << 4);
        payload.push(self.modes[2] as u8 | (self.modes[3] as u8) << 4);

        Payload::<CommandPL>::new(&payload).unwrap()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerTelemetry {
    pub modes: [HeatingMode; 4],
    pub power_status: bool,
}

impl TryFrom<&Payload<TelemetryPL>> for HeatingControllerTelemetry {
    type Error = ProtocolError;

    fn try_from(payload: &Payload<TelemetryPL>) -> Result<Self, ProtocolError> {
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

impl Into<Payload<TelemetryPL>> for HeatingControllerTelemetry {
    fn into(self) -> Payload<TelemetryPL> {
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
