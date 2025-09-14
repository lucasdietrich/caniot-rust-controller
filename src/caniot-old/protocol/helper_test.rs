use chrono::Utc;

use crate::caniot::Payload;

use super::*;

#[test]
fn test_response_match_any_attribute_query() {
    let timestamp = Utc::now();
    let device_id = DeviceId::try_from_u8(1).unwrap();

    // attribute
    let query = Request {
        device_id,
        data: RequestData::AttributeRead { key: 0x0100 },
        timestamp,
    };

    let response = Response {
        device_id,
        data: ResponseData::Attribute {
            key: 0x0100,
            value: 0x12345678,
        },
        timestamp,
    };
    assert!(is_response_to(&query, &response).is_valid_response());

    let response = Response {
        device_id,
        data: ResponseData::Error {
            source: ErrorSource::Attribute(Some(0x0100)),
            error: None,
        },
        timestamp,
    };
    assert!(is_response_to(&query, &response).is_response_error());

    let response = Response {
        device_id,
        data: ResponseData::Error {
            source: ErrorSource::Attribute(None),
            error: None,
        },
        timestamp,
    };
    assert!(is_response_to(&query, &response).is_response_error());

    let response = Response {
        device_id,
        data: ResponseData::Error {
            source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
            error: None,
        },
        timestamp,
    };
    let is_response = is_response_to(&query, &response);
    assert!(is_response.is_error() && !is_response.is_response());

    // telemetry
    let query = Request {
        device_id,
        data: RequestData::Telemetry {
            endpoint: Endpoint::Application2,
        },
        timestamp,
    };

    let response = Response {
        device_id,
        data: ResponseData::Telemetry {
            endpoint: Endpoint::Application2,
            payload: Payload::new_empty(),
        },
        timestamp,
    };
    assert!(is_response_to(&query, &response).is_valid_response());

    let response = Response {
        device_id,
        data: ResponseData::Telemetry {
            endpoint: Endpoint::Application1,
            payload: Payload::new_empty(),
        },
        timestamp,
    };
    let m = is_response_to(&query, &response);
    assert!(!m.is_error() && !m.is_response());

    let response = Response {
        device_id,
        data: ResponseData::Error {
            source: ErrorSource::Telemetry(Endpoint::Application2, None),
            error: None,
        },
        timestamp,
    };
    assert!(is_response_to(&query, &response).is_response_error());

    let response = Response {
        device_id,
        data: ResponseData::Error {
            source: ErrorSource::Telemetry(Endpoint::BoardControl, None),
            error: None,
        },
        timestamp,
    };
    let m = is_response_to(&query, &response);
    assert!(m.is_error() && !m.is_response());
}

#[test]
fn test_requests_concurrency_did() {
    let timestamp = Utc::now();
    let device_id = DeviceId::try_from_u8(1).unwrap();
    let device_id_alt = DeviceId::try_from_u8(2).unwrap();

    let q1 = Request {
        device_id,
        data: RequestData::Telemetry {
            endpoint: Endpoint::BoardControl,
        },
        timestamp,
    };
    let q2 = Request {
        device_id: device_id_alt,
        data: RequestData::Telemetry {
            endpoint: Endpoint::BoardControl,
        },
        timestamp,
    };
    let q3 = Request {
        device_id,
        data: RequestData::Telemetry {
            endpoint: Endpoint::BoardControl,
        },
        timestamp,
    };

    assert!(!are_requests_concurrent(&q1, &q2));
    assert!(are_requests_concurrent(&q1, &q3));
}

#[test]
fn test_requests_concurrency_telemetry() {
    let timestamp = Utc::now();
    let device_id = DeviceId::try_from_u8(1).unwrap();

    let abqt = |e1, e2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::Telemetry { endpoint: e1 },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::Telemetry { endpoint: e2 },
                timestamp,
            },
        )
    };

    assert!(abqt(
        Endpoint::ApplicationDefault,
        Endpoint::ApplicationDefault
    ));
    assert!(!abqt(Endpoint::ApplicationDefault, Endpoint::Application1));
    assert!(!abqt(Endpoint::ApplicationDefault, Endpoint::Application2));
    assert!(!abqt(Endpoint::ApplicationDefault, Endpoint::BoardControl));

    assert!(!abqt(Endpoint::Application1, Endpoint::ApplicationDefault));
    assert!(abqt(Endpoint::Application1, Endpoint::Application1));
    assert!(!abqt(Endpoint::Application1, Endpoint::Application2));
    assert!(!abqt(Endpoint::Application1, Endpoint::BoardControl));

    assert!(!abqt(Endpoint::Application2, Endpoint::ApplicationDefault));
    assert!(!abqt(Endpoint::Application2, Endpoint::Application1));
    assert!(abqt(Endpoint::Application2, Endpoint::Application2));
    assert!(!abqt(Endpoint::Application2, Endpoint::BoardControl));

    assert!(!abqt(Endpoint::BoardControl, Endpoint::ApplicationDefault));
    assert!(!abqt(Endpoint::BoardControl, Endpoint::Application1));
    assert!(!abqt(Endpoint::BoardControl, Endpoint::Application2));
    assert!(abqt(Endpoint::BoardControl, Endpoint::BoardControl));

    let abqc = |e1, e2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::Command {
                    endpoint: e1,
                    payload: Payload::new_empty(),
                },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::Command {
                    endpoint: e2,
                    payload: Payload::new_empty(),
                },
                timestamp,
            },
        )
    };

    assert!(abqc(
        Endpoint::ApplicationDefault,
        Endpoint::ApplicationDefault
    ));
    assert!(!abqc(Endpoint::ApplicationDefault, Endpoint::Application1));
    assert!(!abqc(Endpoint::ApplicationDefault, Endpoint::Application2));
    assert!(!abqc(Endpoint::ApplicationDefault, Endpoint::BoardControl));

    assert!(!abqc(Endpoint::Application1, Endpoint::ApplicationDefault));
    assert!(abqc(Endpoint::Application1, Endpoint::Application1));
    assert!(!abqc(Endpoint::Application1, Endpoint::Application2));
    assert!(!abqc(Endpoint::Application1, Endpoint::BoardControl));

    assert!(!abqc(Endpoint::Application2, Endpoint::ApplicationDefault));
    assert!(!abqc(Endpoint::Application2, Endpoint::Application1));
    assert!(abqc(Endpoint::Application2, Endpoint::Application2));
    assert!(!abqc(Endpoint::Application2, Endpoint::BoardControl));

    assert!(!abqc(Endpoint::BoardControl, Endpoint::ApplicationDefault));
    assert!(!abqc(Endpoint::BoardControl, Endpoint::Application1));
    assert!(!abqc(Endpoint::BoardControl, Endpoint::Application2));
    assert!(abqc(Endpoint::BoardControl, Endpoint::BoardControl));

    let abqtc = |e1, e2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::Telemetry { endpoint: e1 },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::Command {
                    endpoint: e2,
                    payload: Payload::new_empty(),
                },
                timestamp,
            },
        )
    };

    assert!(abqtc(
        Endpoint::ApplicationDefault,
        Endpoint::ApplicationDefault
    ));
    assert!(!abqtc(Endpoint::ApplicationDefault, Endpoint::Application1));
    assert!(!abqtc(Endpoint::ApplicationDefault, Endpoint::Application2));
    assert!(!abqtc(Endpoint::ApplicationDefault, Endpoint::BoardControl));

    assert!(!abqtc(Endpoint::Application1, Endpoint::ApplicationDefault));
    assert!(abqtc(Endpoint::Application1, Endpoint::Application1));
    assert!(!abqtc(Endpoint::Application1, Endpoint::Application2));
    assert!(!abqtc(Endpoint::Application1, Endpoint::BoardControl));

    assert!(!abqtc(Endpoint::Application2, Endpoint::ApplicationDefault));
    assert!(!abqtc(Endpoint::Application2, Endpoint::Application1));
    assert!(abqtc(Endpoint::Application2, Endpoint::Application2));
    assert!(!abqtc(Endpoint::Application2, Endpoint::BoardControl));

    assert!(!abqtc(Endpoint::BoardControl, Endpoint::ApplicationDefault));
    assert!(!abqtc(Endpoint::BoardControl, Endpoint::Application1));
    assert!(!abqtc(Endpoint::BoardControl, Endpoint::Application2));
    assert!(abqtc(Endpoint::BoardControl, Endpoint::BoardControl));
}

#[test]
fn test_requests_concurrency_attributes() {
    let timestamp = Utc::now();
    let device_id = DeviceId::try_from_u8(1).unwrap();

    let abqr = |k1, k2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::AttributeRead { key: k1 },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::AttributeRead { key: k2 },
                timestamp,
            },
        )
    };

    assert!(abqr(0x0100, 0x0100));
    assert!(!abqr(0x0100, 0x0200));

    let abqw = |k1, k2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::AttributeWrite {
                    key: k1,
                    value: 0x12345678,
                },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::AttributeWrite {
                    key: k2,
                    value: 0x12345678,
                },
                timestamp,
            },
        )
    };

    assert!(abqw(0x0100, 0x0100));
    assert!(!abqw(0x0100, 0x0200));

    let abqar = |k1, k2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::AttributeRead { key: k1 },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::AttributeWrite {
                    key: k2,
                    value: 0x12345678,
                },
                timestamp,
            },
        )
    };

    assert!(abqar(0x0100, 0x0100));
    assert!(!abqar(0x0100, 0x0200));

    let abqaw = |k1, k2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: RequestData::AttributeWrite {
                    key: k1,
                    value: 0x12345678,
                },
                timestamp,
            },
            &Request {
                device_id,
                data: RequestData::AttributeRead { key: k2 },
                timestamp,
            },
        )
    };

    assert!(abqaw(0x0100, 0x0100));
    assert!(!abqaw(0x0100, 0x0200));
}

#[test]
fn test_requests_concurrency_any() {
    let timestamp = Utc::now();
    let device_id = DeviceId::try_from_u8(1).unwrap();

    let abq = |d1, d2| {
        are_requests_concurrent(
            &Request {
                device_id,
                data: d1,
                timestamp,
            },
            &Request {
                device_id,
                data: d2,
                timestamp,
            },
        )
    };

    // test combinations of telemetry, command, attribute read and attribute write
    assert!(abq(
        RequestData::Telemetry {
            endpoint: Endpoint::ApplicationDefault
        },
        RequestData::Telemetry {
            endpoint: Endpoint::ApplicationDefault
        }
    ));
    assert!(!abq(
        RequestData::Telemetry {
            endpoint: Endpoint::ApplicationDefault
        },
        RequestData::Telemetry {
            endpoint: Endpoint::Application1
        }
    ));
    assert!(abq(
        RequestData::Telemetry {
            endpoint: Endpoint::ApplicationDefault
        },
        RequestData::Command {
            endpoint: Endpoint::ApplicationDefault,
            payload: Payload::new_empty()
        }
    ));
    assert!(!abq(
        RequestData::Telemetry {
            endpoint: Endpoint::ApplicationDefault
        },
        RequestData::AttributeRead { key: 0x0100 }
    ));
    assert!(!abq(
        RequestData::Telemetry {
            endpoint: Endpoint::ApplicationDefault
        },
        RequestData::AttributeWrite {
            key: 0x0100,
            value: 0x12345678
        }
    ));
}
