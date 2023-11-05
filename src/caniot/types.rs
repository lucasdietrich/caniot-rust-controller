use std::default;

use num_derive::FromPrimitive;

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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SystemCommand {
    pub hardware_reset: bool,
    pub software_reset: bool, // deprecated
    pub watchdog_reset: bool, // deprecated
    pub watchdog_enable: TS,
    pub factory_reset: bool,
    pub inhibit: TSP,
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

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Class0Command {
    pub coc1: Xps,
    pub coc2: Xps,
    pub crl1: Xps,
    pub crl2: Xps,

    pub sys: SystemCommand,
}

pub enum BlcCommand {
    Class0(Class0Command),
    // Class1(Class1Command),
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
