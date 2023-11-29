use crate::{
    caniot::{self},
    controller::{ManagedDeviceTrait, ManagedDeviceError, Device},
};

#[derive(Default)]
pub struct Unmanaged {
}

impl ManagedDeviceTrait for Unmanaged {
    type Error = ManagedDeviceError;

    fn handle_frame(&mut self, frame: &caniot::Response) -> Result<(), Self::Error> {
        Err(ManagedDeviceError::NotImplemented)
    }
}