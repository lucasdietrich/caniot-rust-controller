use std::sync::Arc;

use thiserror::Error;

use super::*;

#[derive(Error, Debug)]
enum DeviceError {
    #[error("Unhandled Request")]
    UnhandledRequest,
}

trait ControllerTrait<E> {
    fn send(&mut self, frame: &Request) -> Result<(), E>;

    fn query(&mut self, request: &Request, timeout_ms: u32) -> Result<Response, E>;
}

trait DeviceHandle<T> {}

trait ManagedDeviceTrait {
    fn process_frame(&self, frame: Option<&Response>) -> Result<(), DeviceError>;

    fn get_device_handle(&self) -> Arc<dyn DeviceHandle<Self>>;
}
