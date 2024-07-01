use super::traits::{Class, ClassCommandTrait, ClassTelemetryTrait};
use crate::caniot::{ClCd, Payload, ProtocolError, Temperature, Ty, Xps};
use num::FromPrimitive;
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize)]
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
    pub poc2: bool,
    pub prl1: bool,
    pub prl2: bool,

    pub temp_in: Temperature,
    pub temp_out: [Temperature; 3],
}

impl ClassTelemetryTrait for Telemetry {
    fn get_board_temperature(&self) -> Option<f32> {
        self.temp_in.to_celsius()
    }

    fn get_outside_temperature(&self) -> Option<f32> {
        // return first valid temperature (get first if multiple are valid)
        self.temp_out.iter().find_map(|t| t.to_celsius())
    }
}

impl TryFrom<&Payload<Ty>> for Telemetry {
    type Error = ProtocolError;

    fn try_from(payload: &Payload<Ty>) -> Result<Self, ProtocolError> {
        let payload = payload.as_ref();
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
                poc2: payload[1] & 0b0000_0010 != 0,
                prl1: payload[1] & 0b0000_0100 != 0,
                prl2: payload[1] & 0b0000_1000 != 0,
                temp_in: Temperature::from_raw_u10(u16::from_le_bytes([
                    payload[2],
                    payload[3] & 0b0000_0011,
                ])),
                temp_out: [
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        (payload[3] >> 2) | (payload[4] & 0b0000_0011) << 6,
                        (payload[4] & 0b0000_1100) >> 2,
                    ])),
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        (payload[4] >> 4) | (payload[5] & 0b0000_1111) << 4,
                        (payload[5] & 0b0011_0000) >> 4,
                    ])),
                    Temperature::from_raw_u10(u16::from_le_bytes([
                        (payload[5] >> 6) | (payload[6] & 0b0011_1111) << 2,
                        (payload[6] & 0b1100_0000) >> 6,
                    ])),
                ],
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

impl Into<Payload<Ty>> for Telemetry {
    fn into(self) -> Payload<Ty> {
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
                | (self.poc2 as u8) << 1
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

        Payload::<Ty>::new(payload).unwrap()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize)]
pub struct Command {
    pub coc1: Xps,
    pub coc2: Xps,
    pub crl1: Xps,
    pub crl2: Xps,
}

impl ClassCommandTrait for Command {
    fn has_effect(&self) -> bool {
        self.coc1 != Xps::None
            || self.coc2 != Xps::None
            || self.crl1 != Xps::None
            || self.crl2 != Xps::None
    }
}

impl Into<Payload<ClCd>> for Command {
    fn into(self) -> Payload<ClCd> {
        let mut payload = vec![0; 7];

        payload[0] = self.coc1 as u8;
        payload[0] |= (self.coc2 as u8) << 3;
        payload[0] |= ((self.crl1 as u8) & 0b11) << 6;
        payload[1] = ((self.crl1 as u8) & 0b100) >> 2;
        payload[1] |= (self.crl2 as u8) << 1;

        Payload::<ClCd>::new(payload).unwrap()
    }
}

impl TryFrom<&Payload<ClCd>> for Command {
    type Error = ProtocolError;

    fn try_from(payload: &Payload<ClCd>) -> Result<Self, ProtocolError> {
        let payload = payload.as_ref();
        if payload.len() >= 2 {
            Ok(Command {
                coc1: FromPrimitive::from_u8(payload[0] & 0b0000_0111).unwrap(),
                coc2: FromPrimitive::from_u8((payload[0] & 0b0011_1000) >> 3).unwrap(),
                crl1: FromPrimitive::from_u8(
                    ((payload[0] & 0b1100_0000) >> 6) | ((payload[1] & 0b0000_0001) << 2),
                )
                .unwrap(),
                crl2: FromPrimitive::from_u8((payload[1] & 0b0000_1110) >> 1).unwrap(),
            })
        } else {
            Err(ProtocolError::PayloadDecodeError)
        }
    }
}

pub struct Class0;

impl Class for Class0 {
    const CLASS_ID: u8 = 0;

    type Telemetry = Telemetry;
    type Command = Command;
}
