use crate::caniot::{self, HeatingMode};

pub struct HeatersController {
    pub heaters: [HeatingMode; 4],
    pub power_status: bool,
}
