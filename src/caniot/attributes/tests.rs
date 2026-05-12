use super::*;

#[test]
fn trait_meta_and_section_are_consistent() {
    assert_eq!(IdNodeIdAttr::META.section, Section::Identification);
}

#[test]
fn parse_config_flags_works() {
    let v = ConfigFlagsAttr::parse_value(AttrKey::new(AttrId::ConfigFlags.key()), 0b1_10_1);

    assert!(v.error_response);
    assert!(!v.telemetry_delay_random);
    assert_eq!(v.telemetry_endpoint, 3);
    assert!(!v.telemetry_periodic_enabled);
}

#[test]
fn enum_references_attr_impls() {
    let parsed = parse_attr_value(AttrId::ConfigTimezone.key(), 3600).unwrap();
    let meta = parsed.get_meta();

    assert_eq!(meta.id, AttrId::ConfigTimezone);
    assert_eq!(meta.display_name, "Device timezone");
}
