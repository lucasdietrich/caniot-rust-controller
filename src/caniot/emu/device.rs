use std::time::{Duration, Instant};

use crate::caniot::{self, Attribute, DeviceId, Endpoint, ErrorCode};

use super::{Behavior, DefaultBehavior};

pub struct Device {
    pub did: DeviceId,
    pub telemetry_interval: Duration, // ms
    pub telemetry_endpoint: Endpoint,

    start_time: Instant,             // ms
    last_telemetry: Option<Instant>, // ms

    pub(super) behavior: Vec<Box<dyn Behavior>>,
}

impl Device {
    pub fn new(id: u8, telemetry_interval: Duration) -> Self {
        let did = DeviceId::from_u8(id).unwrap();

        let mut default_behavior = DefaultBehavior::default();
        default_behavior.set_did(&did);

        Self {
            did,
            telemetry_interval,
            telemetry_endpoint: Endpoint::BoardControl,
            start_time: Instant::now(),
            last_telemetry: None,
            behavior: vec![Box::new(default_behavior)],
        }
    }

    pub fn set_telemetry_endpoint(&mut self, endpoint: Endpoint) {
        self.telemetry_endpoint = endpoint;
    }

    pub fn set_telemetry_interval(&mut self, interval: Duration) {
        self.telemetry_interval = interval;
    }

    fn read_attribute(&self, attr: impl TryInto<Attribute>) -> Option<u32> {
        match attr.try_into() {
            Ok(Attribute::NodeId) => Some(self.did.to_u8() as u32),
            Ok(Attribute::SystemUptime) => Some(self.start_time.elapsed().as_millis() as u32),
            Ok(Attribute::ConfigTelemetryPeriod) => {
                Some(self.telemetry_interval.as_millis() as u32)
            }
            _ => None,
        }
    }

    fn write_attribute(&mut self, attr: impl TryInto<Attribute>, value: u32) -> bool {
        match attr.try_into() {
            Ok(Attribute::ConfigTelemetryPeriod) => {
                self.telemetry_interval = Duration::from_millis(value as u64);
                true
            }
            _ => false,
        }
    }

    pub fn get_remaining_to_telemetry(&self) -> Duration {
        let now = Instant::now();
        if let Some(last_telemetry) = self.last_telemetry {
            let elapsed = now.duration_since(last_telemetry);
            if elapsed < self.telemetry_interval {
                self.telemetry_interval - elapsed
            } else {
                Duration::from_secs(0)
            }
        } else {
            Duration::from_secs(0) // first telemetry
        }
    }

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

    fn handle_command(
        &mut self,
        endpoint: &caniot::Endpoint,
        payload: &[u8],
    ) -> Result<Vec<u8>, caniot::ErrorCode> {
        for behavior in self.behavior.iter_mut().rev() {
            if let Some(error_code) = behavior.on_command(endpoint, payload.to_vec()) {
                if error_code == ErrorCode::Ok {
                    return self.handle_telemetry(endpoint);
                } else {
                    return Err(error_code);
                }
            }
        }

        // No handler found
        Err(ErrorCode::Ehandlerc)
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
                    match self.handle_command(endpoint, payload) {
                        Ok(response) => Some(caniot::ResponseData::Telemetry {
                            endpoint: *endpoint,
                            payload: response,
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
                            payload,
                        }),
                        Err(error) => Some(caniot::ResponseData::Error {
                            source: caniot::ErrorSource::Telemetry(*endpoint, None),
                            error: Some(error),
                        }),
                    }
                }
            }
        } else {
            if self.get_remaining_to_telemetry() == Duration::from_secs(0) {
                let endpoint = self.telemetry_endpoint;
                match self.handle_telemetry(&endpoint) {
                    Ok(payload) => Some(caniot::ResponseData::Telemetry { endpoint, payload }),
                    Err(error) => Some(caniot::ResponseData::Error {
                        source: caniot::ErrorSource::Telemetry(endpoint, None),
                        error: Some(error),
                    }),
                }
            } else {
                None
            }
        }
        .map(|data| caniot::Response {
            device_id: self.did,
            data,
        })
    }
}
