// Tells when a sensor state changed

use crate::controller::DoorState;

pub trait SensorChangeEventTrait: Default {
    fn has_changes(&self) -> bool;

    fn into_option(self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.has_changes() {
            Some(self)
        } else {
            None
        }
    }
}

impl SensorChangeEventTrait for GarageStateChanged {
    fn has_changes(&self) -> bool {
        self.left_door_changed || self.right_door_changed || self.gate_changed
    }
}

#[derive(Debug, Clone, Default)]
pub struct GarageStateChanged {
    pub left_door_changed: bool,
    pub right_door_changed: bool,
    pub gate_changed: bool,

    pub left_door_state: DoorState,
    pub right_door_state: DoorState,
    pub gate_state: DoorState,
}

#[derive(Debug, Clone)]
pub enum SensorChangeEvent {
    GarageStateChanged(GarageStateChanged),
}
