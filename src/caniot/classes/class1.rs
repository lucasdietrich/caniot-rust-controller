use crate::caniot::{ProtocolError, Temperature, Xps};

use super::traits::{Class, ClassCommandTrait, ClassTelemetryTrait};

const CLASS1_IO_COUNT: usize = 19;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Telemetry {
    pub ios: [bool; CLASS1_IO_COUNT],

    pub temp_in: Temperature,
    pub temp_out: [Temperature; 3],
}

impl ClassTelemetryTrait<'_> for Telemetry {}
impl TryFrom<&[u8]> for Telemetry {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 8 {
            Ok(Telemetry {
                ios: [
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
                    payload[2] & 0b0000_0001 != 0, // pb0
                    payload[2] & 0b0000_0010 != 0, // pe0
                    payload[2] & 0b0000_0100 != 0, // pe1
                ],
                temp_in: Temperature::from_raw_u10(u16::from_le_bytes([
                    payload[3],
                    payload[4] & 0b0000_0011,
                ])),
                temp_out: [
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        payload[4] >> 2,
                        payload[5] & 0b0000_1111,
                    ])),
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        payload[5] >> 4,
                        payload[6] & 0b0011_1111,
                    ])),
                    Temperature::from_raw_u10(u16::from_le_bytes([payload[6] >> 6, payload[7]])),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

impl Into<Vec<u8>> for Telemetry {
    fn into(self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(8);

        payload.push(
            (self.ios[0] as u8)
                | (self.ios[1] as u8) << 1
                | (self.ios[2] as u8) << 2
                | (self.ios[3] as u8) << 3
                | (self.ios[4] as u8) << 4
                | (self.ios[5] as u8) << 5
                | (self.ios[6] as u8) << 6
                | (self.ios[7] as u8) << 7,
        );
        payload.push(
            (self.ios[8] as u8)
                | (self.ios[9] as u8) << 1
                | (self.ios[10] as u8) << 2
                | (self.ios[11] as u8) << 3
                | (self.ios[12] as u8) << 4
                | (self.ios[13] as u8) << 5
                | (self.ios[14] as u8) << 6
                | (self.ios[15] as u8) << 7,
        );
        payload.push((self.ios[16] as u8) | (self.ios[17] as u8) << 1 | (self.ios[18] as u8) << 2);
        let temp_in = self.temp_in.to_raw_u10_bytes();
        let temp_out = [
            self.temp_out[0].to_raw_u10_bytes(),
            self.temp_out[1].to_raw_u10_bytes(),
            self.temp_out[2].to_raw_u10_bytes(),
        ];

        payload.push(temp_in[0]);
        payload.push(temp_in[1] | (temp_out[0][0] << 2));
        payload.push(temp_out[0][0] >> 6 | (temp_out[0][1] << 2) | (temp_out[1][0] << 4));
        payload.push(temp_out[1][0] >> 4 | (temp_out[1][1] << 4) | (temp_out[2][0] << 6));
        payload.push(temp_out[2][0] >> 2 | (temp_out[2][1] << 6));

        payload
    }
}

#[repr(u8)]
pub enum Class1CommandFields {
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
    Pb0,
    Ee0,
    Ee1,
}

pub struct Command {
    pub ios: [Xps; CLASS1_IO_COUNT],
}

impl<'a> ClassCommandTrait<'a> for Command {}
impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        let mut payload = vec![0; 7];
        for (i, field) in self.ios.iter().enumerate() {
            field.set_at(&mut payload, i).unwrap();
        }
        payload
    }
}

impl TryFrom<&[u8]> for Command {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 7 {
            Ok(Command {
                ios: [
                    Xps::get_at(payload, Class1CommandFields::Pb0 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Poc0 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Poc1 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Poc2 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Poc3 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Pd0 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Pd1 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Pd2 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Pd3 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio0 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio1 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio2 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio3 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio4 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio5 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio6 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Eio7 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Ee0 as usize).unwrap(),
                    Xps::get_at(payload, Class1CommandFields::Ee1 as usize).unwrap(),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

pub enum Class1 {}

impl Class<'_> for Class1 {
    type Telemetry = Telemetry;
    type Command = Command;
}
