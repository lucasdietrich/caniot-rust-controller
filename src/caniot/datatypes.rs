use std::{
    cmp::{max, min},
    fmt::{Debug, Display},
};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use super::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Temperature(Option<i16>);

impl Temperature {
    pub const INVALID: Temperature = Temperature(None);
    const VALUE_U10_MASK: u16 = 0x3FF;
    const VALUE_U10_INVALID_MARKER: u16 = Self::VALUE_U10_MASK;
    const VALUE_U10_MAX_VALID: u16 = 1000;
    const VALUE_U16_MASK: u16 = 0xFFFF;

    pub fn from_raw_u16(raw: u16) -> Self {
        if raw == Self::VALUE_U16_MASK {
            Self::INVALID
        } else {
            Temperature(Some(raw as i16))
        }
    }

    pub fn from_raw_u10(raw: u16) -> Self {
        if raw > Self::VALUE_U10_MAX_VALID || raw == Self::VALUE_U10_MASK {
            Self::INVALID
        } else {
            Temperature(Some((raw * 10) as i16 - 2800))
        }
    }

    pub fn to_raw_u16(&self) -> u16 {
        match self.0 {
            Some(val) => val as u16,
            None => Self::VALUE_U16_MASK,
        }
    }

    pub fn to_raw_u16_bytes(&self) -> [u8; 2] {
        self.to_raw_u16().to_le_bytes()
    }

    pub fn to_raw_u10(&self) -> u16 {
        match self.0 {
            Some(val) => {
                let val = val / 10;
                let val = max(min(val, 720), -280) as i16;
                (val + 280) as u16
            }
            None => Self::VALUE_U10_INVALID_MARKER,
        }
    }

    pub fn to_raw_u10_bytes(&self) -> [u8; 2] {
        self.to_raw_u10().to_le_bytes()
    }

    pub fn to_celsius(&self) -> Option<f32> {
        match self.0 {
            Some(val) => Some(val as f32 / 100.0),
            None => None,
        }
    }

    pub fn random() -> Self {
        let rand = rand::random::<u16>() % 1001;
        Temperature::from_raw_u10(rand)
    }

    pub fn invalid() -> Self {
        Self::INVALID
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_celsius() {
            Some(val) => write!(f, "{} °C", val),
            None => write!(f, "INVALID °C"),
        }
    }
}

impl Debug for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_celsius() {
            Some(val) => write!(f, "{} °C (raw {})", val, self.0.unwrap()),
            None => write!(f, "INVALID °C"),
        }
    }
}

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
    pub fn set_at(&self, data: &mut [u8], position: usize) -> Result<(), ProtocolError> {
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

    pub fn get_at(data: &[u8], position: usize) -> Result<Self, ProtocolError> {
        let len = data.len();
        let msb_index = position * 3;
        let msb_offset = msb_index & 0x7;
        let msb_rem_size = 8 - msb_offset;
        let byte_n = msb_index >> 3;
        let mut xps = (data[byte_n] >> msb_offset) as u8;

        if msb_rem_size < 3 && (byte_n + 1) < len {
            xps |= data[byte_n + 1] << msb_rem_size;
        }

        match Xps::from_u8(xps) {
            Some(xps) => Ok(xps),
            None => Err(ProtocolError::PayloadDecodeError),
        }
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

#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
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
