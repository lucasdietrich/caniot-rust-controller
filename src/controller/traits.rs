use super::controller::ControllerError;
use crate::caniot as ct;

use async_trait::async_trait;

#[async_trait]
pub trait ControllerAPI {
    async fn query(
        &mut self,
        frame: ct::Request,
        timeout_ms: u32,
    ) -> Result<ct::Response, ControllerError>;

    async fn send(&mut self, frame: ct::Request) -> Result<(), ControllerError>;

    async fn query_telemetry(
        &mut self,
        device_id: ct::DeviceId,
        endpoint: ct::Endpoint,
        timeout_ms: u32,
    ) -> Result<ct::Response, ControllerError> {
        self.query(ct::build_telemetry_request(device_id, endpoint), timeout_ms)
            .await
    }

    async fn query_command(
        &mut self,
        device_id: ct::DeviceId,
        endpoint: ct::Endpoint,
        payload: Vec<u8>,
        timeout_ms: u32,
    ) -> Result<ct::Response, ControllerError> {
        self.query(
            ct::build_command_request(device_id, endpoint, payload),
            timeout_ms,
        )
        .await
    }

    async fn query_attribute_read(
        &mut self,
        device_id: ct::DeviceId,
        attribute: u16,
        timeout_ms: u32,
    ) -> Result<ct::Response, ControllerError> {
        self.query(
            ct::build_attribute_read_request(device_id, attribute),
            timeout_ms,
        )
        .await
    }

    async fn query_attribute_write(
        &mut self,
        device_id: ct::DeviceId,
        attribute: u16,
        value: u32,
        timeout_ms: u32,
    ) -> Result<ct::Response, ControllerError> {
        self.query(
            ct::build_attribute_write_request(device_id, attribute, value),
            timeout_ms,
        )
        .await
    }
}
