use super::*;

#[test]
fn test_parse_error_payload() {
    fn test_payload(
        endpoint: Option<Endpoint>,
        payload: &[u8],
        expected_source: ErrorSource,
        expected_error: Option<ErrorCode>,
    ) {
        let resp = parse_error_payload(endpoint, payload).unwrap();
        assert_eq!(
            resp,
            ResponseData::Error {
                source: expected_source,
                error: expected_error
            }
        );
    }

    test_payload(
        Some(Endpoint::ApplicationDefault),
        &[],
        ErrorSource::Telemetry(Endpoint::ApplicationDefault, None),
        None,
    );

    test_payload(
        Some(Endpoint::ApplicationDefault),
        &[0x00, 0x00, 0x00, 0x00, 0x00],
        ErrorSource::Telemetry(Endpoint::ApplicationDefault, None),
        Some(ErrorCode::Ok),
    );

    test_payload(
        Some(Endpoint::ApplicationDefault),
        &[0x00, 0x3a, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00],
        ErrorSource::Telemetry(Endpoint::ApplicationDefault, Some(0xFF)),
        Some(ErrorCode::Einval),
    );

    test_payload(None, &[0x00], ErrorSource::Attribute(None), None);

    test_payload(
        None,
        &[0x00, 0x3a, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00],
        ErrorSource::Attribute(Some(0x0100)),
        Some(ErrorCode::Einval),
    );
}
