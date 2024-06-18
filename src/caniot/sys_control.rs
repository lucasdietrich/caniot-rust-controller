use num::FromPrimitive;

use super::{Cd, Payload, ProtocolError, RequestData, TS, TSP};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SysCtrl {
    pub hardware_reset: bool,
    pub _software_reset: bool, // deprecated
    pub _watchdog_reset: bool, // deprecated
    pub watchdog_enable: TS,
    pub factory_reset: bool,
    pub inhibit: TSP,
}

impl SysCtrl {
    pub const HARDWARE_RESET: SysCtrl = SysCtrl {
        hardware_reset: true,
        _software_reset: false,
        _watchdog_reset: false,
        watchdog_enable: TS::None,
        factory_reset: false,
        inhibit: TSP::None,
    };

    pub const INHIBIT: SysCtrl = SysCtrl {
        hardware_reset: false,
        _software_reset: false,
        _watchdog_reset: false,
        watchdog_enable: TS::None,
        factory_reset: false,
        inhibit: TSP::Set,
    };

    // Converts the SysCtrl into a class command request
    pub fn into_board_request(self) -> RequestData {
        RequestData::new_board_control_request(self)
    }
}

impl Into<u8> for SysCtrl {
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

impl From<u8> for SysCtrl {
    fn from(value: u8) -> Self {
        SysCtrl {
            hardware_reset: value & 0b0000_0001 != 0,
            _software_reset: value & 0b0000_0010 != 0,
            _watchdog_reset: value & 0b0000_0100 != 0,
            watchdog_enable: FromPrimitive::from_u8((value & 0b0000_1100) >> 2).unwrap(),
            factory_reset: value & 0b0001_0000 != 0,
            inhibit: FromPrimitive::from_u8((value & 0b1100_0000) >> 6).unwrap(),
        }
    }
}

// Implement AsPayload<Cd> for SysCtrl
impl Into<Payload<Cd>> for SysCtrl {
    fn into(self) -> Payload<Cd> {
        Payload::new_from_vec(vec![0, 0, 0, 0, 0, 0, 0, self.into()]).unwrap()
    }
}

impl TryFrom<&Payload<Cd>> for SysCtrl {
    type Error = ProtocolError;

    fn try_from(value: &Payload<Cd>) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            return Err(ProtocolError::ClassCommandSizeError);
        }

        Ok(SysCtrl::from(value.data()[7]))
    }
}

#[cfg(test)]
mod tests {
    use super::SysCtrl;

    #[test]
    fn sys_default() {
        let sys = SysCtrl::default();
        let sys_ser: u8 = sys.into();

        assert_eq!(sys_ser, 0_u8);
    }
}
