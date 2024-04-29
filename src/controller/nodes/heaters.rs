use crate::{
    caniot::{self, HeatingMode},
    controller::{DeviceActionResultTrait, DeviceActionTrait},
};

pub struct HeatersController {
    pub status: HeaterStatus,
}

#[derive(Debug)]
pub struct HeaterStatus {
    pub heaters: [HeatingMode; 4],
    pub power_status: bool,
}

#[derive(Debug)]
pub enum HeaterAction {
    GetStatus,
    SetStatus(Vec<HeatingMode>),
}

impl DeviceActionTrait for HeaterAction {
    type Result = HeaterStatus;
}

impl DeviceActionResultTrait for HeaterStatus {}
