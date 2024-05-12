use super::traits::{Class, ClassCommandTrait, ClassTelemetryTrait};
use crate::caniot::{ProtocolError, Temperature, Xps};
use num::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Telemetry {
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

    pub temp_in: Temperature,
    pub temp_out: [Temperature; 3],
}

impl ClassTelemetryTrait<'_> for Telemetry {}

impl TryFrom<&[u8]> for Telemetry {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 7 {
            Ok(Telemetry {
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
                temp_in: Temperature::from_raw_u10(u16::from_le_bytes([
                    payload[2],
                    payload[3] & 0b0000_0011,
                ])),
                temp_out: [
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        payload[3] >> 2,
                        payload[4] & 0b0000_1111,
                    ])),
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        payload[4] >> 4,
                        payload[5] & 0b0000_1111,
                    ])),
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        payload[5] >> 6,
                        payload[6] & 0b0000_0011,
                    ])),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

impl Into<Vec<u8>> for Telemetry {
    fn into(self) -> Vec<u8> {
        let mut payload = Vec::with_capacity(7);

        payload.push(
            (self.oc1 as u8)
                | (self.oc2 as u8) << 1
                | (self.rl1 as u8) << 2
                | (self.rl2 as u8) << 3
                | (self.in1 as u8) << 4
                | (self.in2 as u8) << 5
                | (self.in3 as u8) << 6
                | (self.in4 as u8) << 7,
        );
        payload.push(
            (self.poc1 as u8)
                | (self.puc2 as u8) << 1
                | (self.prl1 as u8) << 2
                | (self.prl2 as u8) << 3,
        );

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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Command {
    pub coc1: Xps,
    pub coc2: Xps,
    pub crl1: Xps,
    pub crl2: Xps,
}

impl<'a> ClassCommandTrait<'a> for Command {}

impl Into<Vec<u8>> for Command {
    fn into(self) -> Vec<u8> {
        let mut payload = vec![0; 7];

        payload[0] = self.coc1 as u8;
        payload[0] |= (self.coc2 as u8) << 3;
        payload[0] |= ((self.crl1 as u8) & 0b11) << 6;
        payload[1] = ((self.crl1 as u8) & 0b100) >> 2;
        payload[1] |= (self.crl2 as u8) << 1;

        payload
    }
}

impl TryFrom<&[u8]> for Command {
    type Error = ProtocolError;

    fn try_from(payload: &[u8]) -> Result<Self, ProtocolError> {
        if payload.len() >= 2 {
            Ok(Command {
                coc1: FromPrimitive::from_u8(payload[0] & 0b0000_0111).unwrap(),
                coc2: FromPrimitive::from_u8((payload[0] & 0b0011_1000) >> 3).unwrap(),
                crl1: FromPrimitive::from_u8(
                    ((payload[0] & 0b1100_0000) >> 6) | ((payload[1] & 0b0000_0001) << 2),
                )
                .unwrap(),
                crl2: FromPrimitive::from_u8(payload[1] & 0b0000_1110).unwrap(),
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

pub enum Class0 {}

impl Class<'_> for Class0 {
    type Telemetry = Telemetry;
    type Command = Command;
}
