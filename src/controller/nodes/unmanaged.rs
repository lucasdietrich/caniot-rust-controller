use crate::{
    caniot::{self},
    controller::{ManagedDeviceTrait, ManagedDeviceError, Device},
};

#[derive(Default)]
pub struct Unmanaged {
}

impl ManagedDeviceTrait for Unmanaged {
    // type Error = ManagedDeviceError;
}