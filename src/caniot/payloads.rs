use super::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

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
