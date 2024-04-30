use num_traits::FromPrimitive;

use super::*;

trait PayloadTrait<'a>: TryFrom<&'a [u8]> {}

trait CommandTrait: Into<[u8; 7]> {}

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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Class0Payload {
    pub oc1: bool,
    pub oc2: bool,
    pub rl1: bool,
    pub rl2: bool,
    pub in1: bool,
    pub in2: bool,
    pub in3: bool,
    pub in4: bool,
    pub poc1: bool,
    pub puc2: bool,
    pub prl1: bool,
    pub prl2: bool,

    pub temp_in: i16,
    pub temp_out: [i16; 3],
}

impl PayloadTrait<'_> for Class0Payload {}

impl TryFrom<&[u8]> for Class0Payload {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 7 {
            Ok(Class0Payload {
                oc1: payload[0] & 0b0000_0001 != 0,
                oc2: payload[0] & 0b0000_0010 != 0,
                rl1: payload[0] & 0b0000_0100 != 0,
                rl2: payload[0] & 0b0000_1000 != 0,
                in1: payload[0] & 0b0001_0000 != 0,
                in2: payload[0] & 0b0010_0000 != 0,
                in3: payload[0] & 0b0100_0000 != 0,
                in4: payload[0] & 0b1000_0000 != 0,
                poc1: payload[1] & 0b0000_0001 != 0,
                puc2: payload[1] & 0b0000_0010 != 0,
                prl1: payload[1] & 0b0000_0100 != 0,
                prl2: payload[1] & 0b0000_1000 != 0,
                temp_in: i16::from_le_bytes([payload[2], payload[3] & 0b0000_0011]),
                temp_out: [
                    i16::from_le_bytes([payload[3] >> 2, payload[4] & 0b0000_1111]),
                    i16::from_le_bytes([payload[4] >> 4, payload[5] & 0b0000_1111]),
                    i16::from_le_bytes([payload[5] >> 6, payload[6] & 0b0000_0011]),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Class0Command {
    pub coc1: Xps,
    pub coc2: Xps,
    pub crl1: Xps,
    pub crl2: Xps,
}

impl CommandTrait for Class0Command {}
impl Into<[u8; 7]> for Class0Command {
    fn into(self) -> [u8; 7] {
        let mut payload = [0_u8; 7];

        payload[0] = self.coc1 as u8;
        payload[0] |= (self.coc2 as u8) << 3;
        payload[0] |= ((self.crl1 as u8) & 0b11) << 6;
        payload[1] = ((self.crl1 as u8) & 0b100) >> 2;
        payload[1] |= (self.crl2 as u8) << 1;

        payload
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Class1Payload {
    pub ios: [bool; CLASS1_IO_COUNT],

    pub temp_in: i16,
    pub temp_out: [i16; 3],
}

impl PayloadTrait<'_> for Class1Payload {}
impl TryFrom<&[u8]> for Class1Payload {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 7 {
            Ok(Class1Payload {
                ios: [
                    payload[2] & 0b0000_0001 != 0, // pb0
                    payload[0] & 0b0000_0001 != 0, // pc0
                    payload[0] & 0b0000_0010 != 0, // pc1
                    payload[0] & 0b0000_0100 != 0, // pc2
                    payload[0] & 0b0000_1000 != 0, // pc3
                    payload[0] & 0b0001_0000 != 0, // pd0
                    payload[0] & 0b0010_0000 != 0, // pd1
                    payload[0] & 0b0100_0000 != 0, // pd2
                    payload[0] & 0b1000_0000 != 0, // pd3
                    payload[1] & 0b0000_0001 != 0, // eio0
                    payload[1] & 0b0000_0010 != 0, // eio1
                    payload[1] & 0b0000_0100 != 0, // eio2
                    payload[1] & 0b0000_1000 != 0, // eio3
                    payload[1] & 0b0001_0000 != 0, // eio4
                    payload[1] & 0b0010_0000 != 0, // eio5
                    payload[1] & 0b0100_0000 != 0, // eio6
                    payload[1] & 0b1000_0000 != 0, // eio7
                    payload[2] & 0b0000_0010 != 0, // pe0
                    payload[2] & 0b0000_0100 != 0, // pe1
                ],
                temp_in: i16::from_le_bytes([payload[3], payload[4] & 0b0000_0011]),
                temp_out: [
                    i16::from_le_bytes([payload[4] >> 2, payload[5] & 0b0000_1111]),
                    i16::from_le_bytes([payload[5] >> 4, payload[6] & 0b0000_1111]),
                    i16::from_le_bytes([payload[6] >> 6, payload[7] & 0b0000_0011]),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

const CLASS1_IO_COUNT: usize = 19;

#[repr(u8)]
pub enum Class1CommandFields {
    Pb0,
    Poc0,
    Poc1,
    Poc2,
    Poc3,
    Pd0,
    Pd1,
    Pd2,
    Pd3,
    Eio0,
    Eio1,
    Eio2,
    Eio3,
    Eio4,
    Eio5,
    Eio6,
    Eio7,
    Ee0,
    Ee1,
}

pub struct Class1Command {
    pub ios: [Xps; CLASS1_IO_COUNT],
}

impl CommandTrait for Class1Command {}
impl Into<[u8; 7]> for Class1Command {
    fn into(self) -> [u8; 7] {
        let mut payload = [0; 7];
        for (i, field) in self.ios.iter().enumerate() {
            field.set_at(&mut payload, i).unwrap();
        }
        payload
    }
}

pub enum BlcClassCommand {
    Class0(Class0Command),
    Class1(Class1Command),
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
        let payload: [u8; 7] = if let Some(class_payload) = self.class_payload {
            match class_payload {
                BlcClassCommand::Class0(class0_command) => class0_command.into(),
                BlcClassCommand::Class1(class1_command) => class1_command.into(),
            }
        } else {
            [0; 7]
        };

        let payload: [u8; 8] = [
            payload[0],
            payload[1],
            payload[2],
            payload[3],
            payload[4],
            payload[5],
            payload[6],
            self.sys.into(),
        ];
        payload
    }
}

pub enum BlcPayload {
    Class0(Class0Payload),
    Class1(Class1Payload),
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerCommand {
    pub modes: [HeatingMode; 4],
}

impl TryFrom<&[u8]> for HeatingControllerCommand {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 3 {
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

impl TryFrom<&[u8]> for HeatingControllerTelemetry {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
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

impl Into<Vec<u8>> for HeatingControllerTelemetry {
    fn into(self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(3);

        payload.push(self.modes[0] as u8 | (self.modes[1] as u8) << 4);
        payload.push(self.modes[2] as u8 | (self.modes[3] as u8) << 4);
        payload.push(self.power_status as u8);

        payload
    }
}
