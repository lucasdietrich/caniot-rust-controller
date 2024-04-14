use super::{DeviceId, Endpoint, ErrorSource, Request, RequestData, Response, ResponseData};

pub fn build_telemetry_request(device_id: DeviceId, endpoint: Endpoint) -> Request {
    Request {
        device_id,
        data: RequestData::Telemetry { endpoint },
    }
}
pub fn build_attribute_read_request(device_id: DeviceId, key: u16) -> Request {
    Request {
        device_id,
        data: RequestData::AttributeRead { key },
    }
}

pub fn build_attribute_write_request(device_id: DeviceId, key: u16, value: u32) -> Request {
    Request {
        device_id,
        data: RequestData::AttributeWrite { key, value },
    }
}

pub fn build_command_request(device_id: DeviceId, endpoint: Endpoint, payload: Vec<u8>) -> Request {
    Request {
        device_id,
        data: RequestData::Command { endpoint, payload },
    }
}

pub fn build_telemetry_response(
    device_id: DeviceId,
    endpoint: Endpoint,
    payload: Vec<u8>,
) -> Response {
    Response {
        device_id,
        data: ResponseData::Telemetry { endpoint, payload },
    }
}

#[derive(Debug, Clone)]
pub struct ResponseMatch {
    is_reponse: bool,
    is_error: bool,
}

impl ResponseMatch {
    pub fn new(is_response: bool, is_error: bool) -> Self {
        Self {
            is_reponse: is_response,
            is_error: is_error,
        }
    }

    pub fn is_response(&self) -> bool {
        self.is_reponse
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn is_valid_response(&self) -> bool {
        self.is_reponse && !self.is_error
    }

    pub fn is_response_error(&self) -> bool {
        self.is_reponse && self.is_error
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

///
/// Tests private functions
///

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_response_match_any_attribute_query() {
        // attribute
        let query = Request {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: RequestData::AttributeRead { key: 0x0100 },
        };

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Attribute {
                key: 0x0100,
                value: 0x12345678,
            },
        };
        assert!(is_response_to(&query, &response).is_valid_response());

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Error {
                source: ErrorSource::Attribute(Some(0x0100)),
                error: None,
            },
        };
        assert!(is_response_to(&query, &response).is_response_error());

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Error {
                source: ErrorSource::Attribute(None),
                error: None,
            },
        };
        assert!(is_response_to(&query, &response).is_response_error());

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Error {
                source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
                error: None,
            },
        };
        let is_response = is_response_to(&query, &response);
        assert!(is_response.is_error() && !is_response.is_response());

        // telemetry
        let query = Request {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: RequestData::Telemetry {
                endpoint: Endpoint::Application2,
            },
        };

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Telemetry {
                endpoint: Endpoint::Application2,
                payload: vec![],
            },
        };
        assert!(is_response_to(&query, &response).is_valid_response());

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Telemetry {
                endpoint: Endpoint::Application1,
                payload: vec![],
            },
        };
        let m = is_response_to(&query, &response);
        assert!(!m.is_error() && !m.is_response());

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Error {
                source: ErrorSource::Telemetry(Endpoint::Application2, None),
                error: None,
            },
        };
        assert!(is_response_to(&query, &response).is_response_error());

        let response = Response {
            device_id: DeviceId::from_u8(1).unwrap(),
            data: ResponseData::Error {
                source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
                error: None,
            },
        };
        let m = is_response_to(&query, &response);
        assert!(m.is_error() && !m.is_response());
    }
}
