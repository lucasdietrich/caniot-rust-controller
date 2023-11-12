use super::super::types::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorCommand {
    // #[crl1(PulseOn)]
    pub left_door_activate: bool,
    // #[crl2(PulseOn)]
    pub right_door_activate: bool,
}

#[allow(clippy::all)]
impl Into<Class0Command> for GarageDoorCommand {
    fn into(self) -> Class0Command {
        Class0Command {
            crl1: if self.left_door_activate {
                Xps::PulseOn
            } else {
                Xps::None
            },
            crl2: if self.right_door_activate {
                Xps::PulseOn
            } else {
                Xps::None
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GarageDoorStatus {
    pub left_door_status: bool,
    pub right_door_status: bool,
    pub garage_light_status: bool,
}

impl From<Class0Payload> for GarageDoorStatus {
    fn from(payload: Class0Payload) -> Self {
        Self {
            left_door_status: payload.in3,
            right_door_status: payload.in4,
            garage_light_status: payload.in2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enc() {
        let cmd = GarageDoorCommand {
            left_door_activate: true,
            right_door_activate: true,
        };
        let cmd: Class0Command = cmd.into();
        assert_eq!(cmd.crl1, Xps::PulseOn);
        assert_eq!(cmd.crl2, Xps::PulseOn);
        assert_eq!(cmd.coc1, Xps::None);
        assert_eq!(cmd.coc2, Xps::None);
    }

    #[test]
    fn dec() {
        let payload = Class0Payload {
            in2: true,
            in3: true,
            in4: true,
            ..Default::default()
        };
        let status = GarageDoorStatus::from(payload);
        assert_eq!(status.left_door_status, true);
        assert_eq!(status.right_door_status, true);
        assert_eq!(status.garage_light_status, true);
    }
}
