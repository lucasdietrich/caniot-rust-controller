use super::*;
#[test]
fn test_response_match_any_attribute_query() {
    // attribute
    let query = Request {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: RequestData::AttributeRead { key: 0x0100 },
    };

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Attribute {
            key: 0x0100,
            value: 0x12345678,
        },
    };
    assert!(is_response_to(&query, &response).is_valid_response());

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Error {
            source: ErrorSource::Attribute(Some(0x0100)),
            error: None,
        },
    };
    assert!(is_response_to(&query, &response).is_response_error());

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Error {
            source: ErrorSource::Attribute(None),
            error: None,
        },
    };
    assert!(is_response_to(&query, &response).is_response_error());

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Error {
            source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
            error: None,
        },
    };
    let is_response = is_response_to(&query, &response);
    assert!(is_response.is_error() && !is_response.is_response());

    // telemetry
    let query = Request {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: RequestData::Telemetry {
            endpoint: Endpoint::Application2,
        },
    };

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Telemetry {
            endpoint: Endpoint::Application2,
            payload: Payload::new_empty(),
        },
    };
    assert!(is_response_to(&query, &response).is_valid_response());

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Telemetry {
            endpoint: Endpoint::Application1,
            payload: Payload::new_empty(),
        },
    };
    let m = is_response_to(&query, &response);
    assert!(!m.is_error() && !m.is_response());

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Error {
            source: ErrorSource::Telemetry(Endpoint::Application2, None),
            error: None,
        },
    };
    assert!(is_response_to(&query, &response).is_response_error());

    let response = Response {
        device_id: DeviceId::try_from_u8(1).unwrap(),
        data: ResponseData::Error {
            source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
            error: None,
        },
    };
    let m = is_response_to(&query, &response);
    assert!(m.is_error() && !m.is_response());
}
