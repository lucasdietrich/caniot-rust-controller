use std::time::{Duration, Instant};

use crate::{
    caniot::{self, Attribute, DeviceId, Endpoint, ErrorCode, Payload},
    grpcserver::EmuEvent,
    utils::expirable::{ttl, ExpirableTrait},
};

use super::{Behavior, BoardControlBehavior};

pub struct Device {
    pub did: DeviceId,

    pub telemetries_requested: Vec<Endpoint>,
    pub telemetry_interval: Option<Duration>, // ms
    pub telemetry_endpoint: Endpoint,

    start_time: Instant,             // ms
    last_telemetry: Option<Instant>, // ms

    // Flag to request an immediate process of the device
    process_requested: bool,

    // Ordered list of behaviors the device implements
    pub(super) behavior: Vec<Box<dyn Behavior>>,
}

impl Device {
    pub fn new(id: u8, telemetry_interval: Option<Duration>) -> Self {
        let did = DeviceId::from_u8(id);

        let mut board_behavior = BoardControlBehavior::default();
        board_behavior.set_did(&did);

        Self {
            did,
            telemetries_requested: vec![Endpoint::BoardControl],
            telemetry_interval,
            telemetry_endpoint: Endpoint::BoardControl,
            start_time: Instant::now(),
            last_telemetry: None,
            process_requested: false,
            behavior: vec![Box::new(board_behavior)],
        }
    }

    pub fn set_telemetry_endpoint(&mut self, endpoint: Endpoint) {
        self.telemetry_endpoint = endpoint;
    }

    pub fn set_telemetry_interval(&mut self, interval: Option<Duration>) {
        self.telemetry_interval = interval;
    }

    pub fn request_telemetry(&mut self, endpoint: Endpoint) {
        self.telemetries_requested.push(endpoint);
    }

    fn read_attribute(&self, attr: impl TryInto<Attribute>) -> Option<u32> {
        match attr.try_into() {
            Ok(Attribute::NodeId) => Some(self.did.to_u8() as u32),
            Ok(Attribute::SystemUptime) => Some(self.start_time.elapsed().as_millis() as u32),
            Ok(Attribute::ConfigTelemetryPeriod) => {
                self.telemetry_interval.map(|ms| ms.as_millis() as u32)
            }
            _ => None,
        }
    }

    fn write_attribute(&mut self, attr: impl TryInto<Attribute>, value: u32) -> bool {
        match attr.try_into() {
            Ok(Attribute::ConfigTelemetryPeriod) => {
                self.set_telemetry_interval(Some(Duration::from_millis(value as u64)));
                true
            }
            _ => false,
        }
    }

    fn get_time_to_next_periodic_telemetry(&self) -> Option<Duration> {
        if let Some(interval) = self.telemetry_interval {
            let now = Instant::now();
            if let Some(last_telemetry) = self.last_telemetry {
                let elapsed = now.duration_since(last_telemetry);
                if elapsed < interval {
                    Some(interval - elapsed)
                } else {
                    Some(Duration::from_secs(0))
                }
            } else {
                Some(Duration::from_secs(0)) // first telemetry
            }
        } else {
            None
        }
    }

    pub fn get_time_to_next_device_process(&self) -> Option<Duration> {
        ttl(&[
            self.behavior.ttl().map(Duration::from_millis),
            self.get_time_to_next_periodic_telemetry(),
        ])
    }

    // Handle telemetry by calling behavior's on_telemetry method in reverse order
    // until a handler returns a payload.
    // If no handler is found, return an error.
    //
    // # Arguments
    // * `endpoint` - The endpoint of the telemetry
    //
    // # Returns
    // * Ok(payload) - The telemetry payload
    // * Err(error) - The error code
    fn handle_telemetry(
        &mut self,
        endpoint: &caniot::Endpoint,
    ) -> Result<Vec<u8>, caniot::ErrorCode> {
        self.last_telemetry = Some(Instant::now());

        for behavior in self.behavior.iter_mut().rev() {
            if let Some(payload) = behavior.on_telemetry(endpoint) {
                return Ok(payload);
            }
        }

        // No handler found
        Err(ErrorCode::Ehandlert)
    }

    // Handle command by calling behavior's on_command method in order
    // until a handler returns an error or a terminate flag is set.
    // If no handler is found, return an error.
    //
    // If at least one handler is found and all of them return Ok,
    // call handle_telemetry to get the telemetry payload.
    //
    // # Arguments
    // * `endpoint` - The endpoint of the command
    // * `payload` - The payload of the command
    //
    // # Returns
    // * Ok(payload) - The telemetry payload
    // * Err(error) - The error code
    fn handle_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        payload: &[u8],
    ) -> Result<Vec<u8>, caniot::ErrorCode> {
        let mut command_processed_once = false;
        let mut terminate = false;

        for behavior in self.behavior.iter_mut() {
            if let Some(error_code) =
                behavior.on_command(endpoint, payload.to_vec(), &mut terminate)
            {
                command_processed_once = true;

                if error_code != ErrorCode::Ok {
                    return Err(error_code);
                }

                if !terminate {
                    break;
                }
            }
        }

        if command_processed_once {
            self.handle_telemetry(endpoint)
        } else {
            // No handler found
            Err(ErrorCode::Ehandlerc)
        }
    }

    pub fn process(&mut self, req: Option<&caniot::RequestData>) -> Option<caniot::Response> {
        if let Some(req) = req {
            match req {
                caniot::RequestData::AttributeRead { key } => {
                    if let Some(value) = self.read_attribute(*key) {
                        Some(caniot::ResponseData::Attribute { key: *key, value })
                    } else {
                        Some(caniot::ResponseData::Error {
                            source: caniot::ErrorSource::Attribute(Some(*key)),
                            error: Some(ErrorCode::Enoattr),
                        })
                    }
                }
                caniot::RequestData::AttributeWrite { key, value } => {
                    if self.write_attribute(*key, *value) {
                        Some(caniot::ResponseData::Attribute {
                            key: *key,
                            value: *value,
                        })
                    } else {
                        Some(caniot::ResponseData::Error {
                            source: caniot::ErrorSource::Attribute(Some(*key)),
                            error: Some(ErrorCode::Ereadonly),
                        })
                    }
                }
                caniot::RequestData::Command { endpoint, payload } => {
                    match self.handle_command(endpoint, payload.as_ref()) {
                        Ok(response) => Some(caniot::ResponseData::Telemetry {
                            endpoint: *endpoint,
                            payload: Payload::new_unchecked(&response),
                        }),
                        Err(error) => Some(caniot::ResponseData::Error {
                            source: caniot::ErrorSource::Telemetry(*endpoint, None),
                            error: Some(error),
                        }),
                    }
                }
                caniot::RequestData::Telemetry { endpoint } => {
                    match self.handle_telemetry(endpoint) {
                        Ok(payload) => Some(caniot::ResponseData::Telemetry {
                            endpoint: *endpoint,
                            payload: Payload::new_unchecked(&payload),
                        }),
                        Err(error) => Some(caniot::ResponseData::Error {
                            source: caniot::ErrorSource::Telemetry(*endpoint, None),
                            error: Some(error),
                        }),
                    }
                }
            }
        } else if self.behavior.expired() || self.process_requested {
            self.process_requested = false;
            let endpoints: Vec<caniot::Endpoint> = self
                .behavior
                .iter_mut()
                .rev()
                .filter_map(|b| b.process())
                .collect();

            for endpoint in endpoints {
                self.request_telemetry(endpoint);
            }
            None
        } else if let Some(endpoint) = self.telemetries_requested.pop() {
            match self.handle_telemetry(&endpoint) {
                Ok(payload) => Some(caniot::ResponseData::Telemetry {
                    endpoint,
                    payload: Payload::new_unchecked(&payload),
                }),
                Err(error) => Some(caniot::ResponseData::Error {
                    source: caniot::ErrorSource::Telemetry(endpoint, None),
                    error: Some(error),
                }),
            }
        } else if self.get_time_to_next_periodic_telemetry() == Some(Duration::from_secs(0)) {
            let endpoint = self.telemetry_endpoint;
            match self.handle_telemetry(&endpoint) {
                Ok(payload) => Some(caniot::ResponseData::Telemetry {
                    endpoint,
                    payload: Payload::new_unchecked(&payload),
                }),
                Err(error) => Some(caniot::ResponseData::Error {
                    source: caniot::ErrorSource::Telemetry(endpoint, None),
                    error: Some(error),
                }),
            }
        } else {
            None
        }
        .map(|data| caniot::Response {
            device_id: self.did,
            data,
        })
    }

    pub fn handle_emu_event(&mut self, event: EmuEvent) {
        for behavior in self.behavior.iter_mut() {
            self.process_requested |= behavior.on_emu_event(event);
        }
    }
}
