use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive)]
pub enum Xps {
    #[default]
    None = 0,
    SetOn = 1,
    SetOff = 2,
    Toggle = 3,
    Reset = 4,
    PulseOn = 5,
    PulseOff = 6,
    PulseCancel = 7,
}

impl Xps {
    fn set_at(&self, data: &mut [u8], position: usize) -> Result<(), ProtocolError> {
        let len = data.len();
        let msb_index = position * 3;
        let msb_offset = msb_index & 0x7;
        let msb_rem_size = 8 - msb_offset;
        let byte_n = msb_index >> 3;
        let xps = *self as u8;
        data[byte_n] |= (xps << msb_offset) as u8;

        if msb_rem_size < 3 && (byte_n + 1) < len {
            data[byte_n + 1] |= xps >> msb_rem_size;
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive)]
pub enum TS {
    #[default]
    None = 0,
    Set = 1,
    Reset = 2,
    Toggle = 3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive)]
pub enum TSP {
    #[default]
    None = 0,
    Set = 1,
    Reset = 2,
    Pulse = 3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive)]
pub enum HeatingMode {
    #[default]
    None = 0,
    Comfort = 1,
    ComfortMin1 = 2,
    ComfortMin2 = 3,
    EnergySaving = 4,
    FrostProtection = 5,
    Stop = 6,
    // unused
}

trait TelemetryPayload<'a>: TryFrom<&'a [u8]> {}

trait TelemetryCommand: Into<[u8; 7]> {}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SystemCommand {
    pub hardware_reset: bool,
    pub _software_reset: bool, // deprecated
    pub _watchdog_reset: bool, // deprecated
    pub watchdog_enable: TS,
    pub factory_reset: bool,
    pub inhibit: TSP,
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

impl TelemetryPayload<'_> for Class0Payload {}

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

impl TelemetryCommand for Class0Command {}
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

impl TelemetryPayload<'_> for Class1Payload {}
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

impl TelemetryCommand for Class1Command {}
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
    pub class_payload: BlcClassCommand,
    pub sys: SystemCommand,
}

impl Into<[u8; 8]> for BlcCommand {
    fn into(self) -> [u8; 8] {
        let payload: [u8; 7] = match self.class_payload {
            BlcClassCommand::Class0(class0) => class0.into(),
            BlcClassCommand::Class1(class1) => class1.into(),
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct HeatingControllerPayload {
    pub modes: [HeatingMode; 4],
    pub power_status: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn set_xps() {
        use super::Xps;
        fn test(xps: Xps, len: usize, pos: usize, expected: &[u8]) {
            let mut data = [0_u8; 8];
            let data = &mut data[..len];
            xps.set_at(data, pos).unwrap();
            assert_eq!(&data[..len], expected);
        }

        test(Xps::None, 1, 1, &[0b0000_0000]);
        test(Xps::PulseCancel, 1, 0, &[0b0000_0111]);
        test(Xps::PulseCancel, 1, 1, &[0b0011_1000]);
        test(Xps::PulseCancel, 1, 2, &[0b1100_0000]);
        test(Xps::PulseCancel, 2, 2, &[0b1100_0000, 0b0000_0001]);
        test(Xps::PulseCancel, 2, 3, &[0b0000_0000, 0b0000_1110]);
        test(Xps::PulseCancel, 2, 4, &[0b0000_0000, 0b0111_0000]);
        test(Xps::PulseCancel, 2, 5, &[0b0000_0000, 0b1000_0000]);
        test(
            Xps::PulseCancel,
            3,
            5,
            &[0b0000_0000, 0b1000_0000, 0b0000_0011],
        );
        test(
            Xps::PulseCancel,
            3,
            6,
            &[0b0000_0000, 0b0000_0000, 0b0001_1100],
        );
        test(
            Xps::PulseCancel,
            3,
            7,
            &[0b0000_0000, 0b0000_0000, 0b1110_0000],
        );
    }
}
