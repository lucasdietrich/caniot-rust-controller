use num::FromPrimitive;
use serde::Serialize;

use super::{class0, class1, HeatingMode, ProtocolError, TS, TSP};

trait TelemetryTrait<'a>: TryFrom<&'a [u8]> + Into<Vec<u8>> {}

trait CommandTrait<'a>: Into<Vec<u8>> + TryFrom<&'a [u8]> {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SystemCommand {
    pub hardware_reset: bool,
    pub _software_reset: bool, // deprecated
    pub _watchdog_reset: bool, // deprecated
    pub watchdog_enable: TS,
    pub factory_reset: bool,
    pub inhibit: TSP,
}

impl SystemCommand {
    pub const HARDWARE_RESET: SystemCommand = SystemCommand {
        hardware_reset: true,
        _software_reset: false,
        _watchdog_reset: false,
        watchdog_enable: TS::None,
        factory_reset: false,
        inhibit: TSP::None,
    };
}

impl Into<u8> for SystemCommand {
    fn into(self) -> u8 {
        let mut payload = 0_u8;

        payload |= self.hardware_reset as u8;
        payload |= (self._software_reset as u8) << 1;
        payload |= (self._watchdog_reset as u8) << 2;
        payload |= (self.watchdog_enable as u8) << 3;
        payload |= (self.factory_reset as u8) << 5;
        payload |= (self.inhibit as u8) << 6;

        payload
    }
}

impl From<u8> for SystemCommand {
    fn from(value: u8) -> Self {
        SystemCommand {
            hardware_reset: value & 0b0000_0001 != 0,
            _software_reset: value & 0b0000_0010 != 0,
            _watchdog_reset: value & 0b0000_0100 != 0,
            watchdog_enable: FromPrimitive::from_u8((value & 0b0000_1100) >> 2).unwrap(),
            factory_reset: value & 0b0001_0000 != 0,
            inhibit: FromPrimitive::from_u8((value & 0b1100_0000) >> 6).unwrap(),
        }
    }
}

pub enum BlcClassCommand {
    Class0(class0::Command),
    Class1(class1::Command),
}

impl Into<[u8; 7]> for BlcClassCommand {
    fn into(self) -> [u8; 7] {
        let mut vec: Vec<_> = match self {
            BlcClassCommand::Class0(class0_command) => class0_command.into(),
            BlcClassCommand::Class1(class1_command) => class1_command.into(),
        };

        if vec.len() > 7 {
            panic!("Class command size error");
        } else if vec.len() < 7 {
            // fill
            vec.resize(7, 0);
        }

        vec.try_into().unwrap()
    }
}

pub struct BlcCommand {
    pub class_payload: Option<BlcClassCommand>,
    pub sys: SystemCommand,
}

impl BlcCommand {
    pub const HARDWARE_RESET: BlcCommand = BlcCommand {
        class_payload: None,
        sys: SystemCommand::HARDWARE_RESET,
    };
}

impl Into<[u8; 8]> for BlcCommand {
    fn into(self) -> [u8; 8] {
        let class_command: [u8; 7] = if let Some(class_command) = self.class_payload {
            class_command.into()
        } else {
            [0; 7]
        };

        let command: [u8; 8] = [
            class_command[0],
            class_command[1],
            class_command[2],
            class_command[3],
            class_command[4],
            class_command[5],
            class_command[6],
            self.sys.into(),
        ];
        command
    }
}

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

impl Into<Vec<u8>> for BlcClassTelemetry {
    fn into(self) -> Vec<u8> {
        match self {
            BlcClassTelemetry::Class0(class0_telemetry) => class0_telemetry.into(),
            BlcClassTelemetry::Class1(class1_telemetry) => class1_telemetry.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerCommand {
    pub modes: [HeatingMode; 4],
}

impl<'a> CommandTrait<'a> for HeatingControllerCommand {}

impl TryFrom<&[u8]> for HeatingControllerCommand {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
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

impl Into<Vec<u8>> for HeatingControllerCommand {
    fn into(self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(3);

        payload.push(self.modes[0] as u8 | (self.modes[1] as u8) << 4);
        payload.push(self.modes[2] as u8 | (self.modes[3] as u8) << 4);

        payload
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerTelemetry {
    pub modes: [HeatingMode; 4],
    pub power_status: bool,
}

impl<'a> TelemetryTrait<'a> for HeatingControllerTelemetry {}
impl TryFrom<&[u8]> for HeatingControllerTelemetry {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 3 {
            Ok(HeatingControllerTelemetry {
                modes: [
                    HeatingMode::from_u8(payload[0] & 0xf)
                        .ok_or(ProtocolError::PayloadDecodeError)?,
                    HeatingMode::from_u8((payload[0] & 0xf0) >> 4)
                        .ok_or(ProtocolError::PayloadDecodeError)?,
                    HeatingMode::from_u8(payload[1] & 0xf)
                        .ok_or(ProtocolError::PayloadDecodeError)?,
                    HeatingMode::from_u8((payload[1] & 0xf0) >> 4)
                        .ok_or(ProtocolError::PayloadDecodeError)?,
                ],
                power_status: payload[2] & 0b0000_0001 != 0,
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
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
