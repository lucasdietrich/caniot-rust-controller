use super::{
    DeviceId, Endpoint, ErrorSource, Payload, Request, RequestData, Response, ResponseData,
};

pub fn build_telemetry_request_data(endpoint: Endpoint) -> RequestData {
    RequestData::Telemetry { endpoint }
}

pub fn build_attribute_read_request_data(key: u16) -> RequestData {
    RequestData::AttributeRead { key }
}

pub fn build_attribute_write_request_data(key: u16, value: u32) -> RequestData {
    RequestData::AttributeWrite { key, value }
}

pub fn build_command_request_data(endpoint: Endpoint, payload: Vec<u8>) -> RequestData {
    RequestData::Command {
        endpoint,
        payload: Payload::new(&payload).unwrap(),
    }
}

pub fn build_telemetry_request(device_id: DeviceId, endpoint: Endpoint) -> Request {
    Request::new(device_id, build_telemetry_request_data(endpoint))
}

pub fn build_attribute_read_request(device_id: DeviceId, key: u16) -> Request {
    Request::new(device_id, build_attribute_read_request_data(key))
}

pub fn build_attribute_write_request(device_id: DeviceId, key: u16, value: u32) -> Request {
    Request::new(device_id, build_attribute_write_request_data(key, value))
}

pub fn build_command_request(device_id: DeviceId, endpoint: Endpoint, payload: Vec<u8>) -> Request {
    Request::new(device_id, build_command_request_data(endpoint, payload))
}

#[derive(Debug, Clone)]
pub struct ResponseMatch {
    is_response: bool,
    is_error: bool,
}

impl ResponseMatch {
    pub fn new(is_response: bool, is_error: bool) -> Self {
        Self {
            is_response,
            is_error,
        }
    }

    pub fn is_response(&self) -> bool {
        self.is_response
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn is_valid_response(&self) -> bool {
        self.is_response && !self.is_error
    }

    pub fn is_response_error(&self) -> bool {
        self.is_response && self.is_error
    }
}

fn response_match_any_telemetry_query(
    query_endpoint: Endpoint,
    response: &Response,
) -> ResponseMatch {
    let (is_response, is_error) = match response.data {
        ResponseData::Telemetry {
            endpoint: response_endpoint,
            ..
        } => (query_endpoint == response_endpoint, false),
        ResponseData::Error {
            source: ErrorSource::Telemetry(endpoint, _),
            ..
        } => (query_endpoint == endpoint, true),
        ResponseData::Error { .. } => (false, true),
        ResponseData::Attribute { .. } => (false, false),
    };

    ResponseMatch::new(is_response, is_error)
}

fn response_match_any_attribute_query(key: u16, response: &Response) -> ResponseMatch {
    let (is_response, is_error) = match response.data {
        ResponseData::Telemetry { .. } => (false, false),
        ResponseData::Attribute {
            key: response_key, ..
        } => (key == response_key, false),
        ResponseData::Error {
            source: ErrorSource::Attribute(err_key),
            ..
        } => (
            // unwrap_or(true) because if no key is present, we assume it is a response to any attribute query
            err_key.map(|err_key| key == err_key).unwrap_or(true),
            true,
        ),
        ResponseData::Error { .. } => (false, true),
    };

    ResponseMatch::new(is_response, is_error)
}

pub fn is_response_to(query: &Request, response: &Response) -> ResponseMatch {
    if query.device_id != DeviceId::BROADCAST && query.device_id != response.device_id {
        return ResponseMatch::new(false, false);
    }

    match query.data {
        RequestData::Command { endpoint, .. } | RequestData::Telemetry { endpoint } => {
            response_match_any_telemetry_query(endpoint, response)
        }
        RequestData::AttributeWrite { key, .. } | RequestData::AttributeRead { key } => {
            response_match_any_attribute_query(key, response)
        }
    }
}
