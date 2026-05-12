use std::fmt;

use super::core::{AttrId, AttrKey, AttrMeta, Attribute};
use super::defs::*;

pub enum ParsedAttributeValue {
    IdNodeId(<IdNodeIdAttr as Attribute>::Value),
    IdVersion(<IdVersionAttr as Attribute>::Value),
    IdName(<IdNameAttr as Attribute>::Value),
    IdMagicNumber(<IdMagicNumberAttr as Attribute>::Value),
    IdBuildDate(<IdBuildDateAttr as Attribute>::Value),
    IdBuildCommit(<IdBuildCommitAttr as Attribute>::Value),
    IdFeatures(<IdFeaturesAttr as Attribute>::Value),
    SystemUptimeSynced(<SystemUptimeSyncedAttr as Attribute>::Value),
    SystemTime(<SystemTimeAttr as Attribute>::Value),
    SystemUptime(<SystemUptimeAttr as Attribute>::Value),
    SystemStartTime(<SystemStartTimeAttr as Attribute>::Value),
    SystemLastTelemetry(<SystemLastTelemetryAttr as Attribute>::Value),
    SystemReceivedTotal(<SystemReceivedTotalAttr as Attribute>::Value),
    SystemReceivedReadAttribute(<SystemReceivedReadAttributeAttr as Attribute>::Value),
    SystemReceivedWriteAttribute(<SystemReceivedWriteAttributeAttr as Attribute>::Value),
    SystemReceivedCommand(<SystemReceivedCommandAttr as Attribute>::Value),
    SystemReceivedRequestTelemetry(<SystemReceivedRequestTelemetryAttr as Attribute>::Value),
    SystemReceivedIgnored(<SystemReceivedIgnoredAttr as Attribute>::Value),
    SystemLastTelemetryMsMod(<SystemLastTelemetryMsModAttr as Attribute>::Value),
    SystemSentTotal(<SystemSentTotalAttr as Attribute>::Value),
    SystemSentTelemetry(<SystemSentTelemetryAttr as Attribute>::Value),
    SystemUnused4(<SystemUnused4Attr as Attribute>::Value),
    SystemLastCommandError(<SystemLastCommandErrorAttr as Attribute>::Value),
    SystemLastTelemetryError(<SystemLastTelemetryErrorAttr as Attribute>::Value),
    SystemUnused5(<SystemUnused5Attr as Attribute>::Value),
    SystemBattery(<SystemBatteryAttr as Attribute>::Value),
    ConfigTelemetryPeriod(<ConfigTelemetryPeriodAttr as Attribute>::Value),
    ConfigTelemetryDelay(<ConfigTelemetryDelayAttr as Attribute>::Value),
    ConfigTelemetryDelayMin(<ConfigTelemetryDelayMinAttr as Attribute>::Value),
    ConfigTelemetryDelayMax(<ConfigTelemetryDelayMaxAttr as Attribute>::Value),
    ConfigFlags(<ConfigFlagsAttr as Attribute>::Value),
    ConfigTimezone(<ConfigTimezoneAttr as Attribute>::Value),
    ConfigLocation(<ConfigLocationAttr as Attribute>::Value),
    ConfigCls0GpioPulseDurationOc1(<ConfigCls0GpioPulseDurationOc1Attr as Attribute>::Value),
    ConfigCls0GpioPulseDurationOc2(<ConfigCls0GpioPulseDurationOc2Attr as Attribute>::Value),
    ConfigCls0GpioPulseDurationRl1(<ConfigCls0GpioPulseDurationRl1Attr as Attribute>::Value),
    ConfigCls0GpioPulseDurationRl2(<ConfigCls0GpioPulseDurationRl2Attr as Attribute>::Value),
    ConfigCls0GpioOutputsDefault(<ConfigCls0GpioOutputsDefaultAttr as Attribute>::Value),
    ConfigCls0GpioTelemetryOnChange(<ConfigCls0GpioTelemetryOnChangeAttr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPc0(<ConfigCls1GpioPulseDurationPc0Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPc1(<ConfigCls1GpioPulseDurationPc1Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPc2(<ConfigCls1GpioPulseDurationPc2Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPc3(<ConfigCls1GpioPulseDurationPc3Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPd0(<ConfigCls1GpioPulseDurationPd0Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPd1(<ConfigCls1GpioPulseDurationPd1Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPd2(<ConfigCls1GpioPulseDurationPd2Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPd3(<ConfigCls1GpioPulseDurationPd3Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei0(<ConfigCls1GpioPulseDurationPei0Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei1(<ConfigCls1GpioPulseDurationPei1Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei2(<ConfigCls1GpioPulseDurationPei2Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei3(<ConfigCls1GpioPulseDurationPei3Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei4(<ConfigCls1GpioPulseDurationPei4Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei5(<ConfigCls1GpioPulseDurationPei5Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei6(<ConfigCls1GpioPulseDurationPei6Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPei7(<ConfigCls1GpioPulseDurationPei7Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPb0(<ConfigCls1GpioPulseDurationPb0Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPe0(<ConfigCls1GpioPulseDurationPe0Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationPe1(<ConfigCls1GpioPulseDurationPe1Attr as Attribute>::Value),
    ConfigCls1GpioPulseDurationReserved(
        <ConfigCls1GpioPulseDurationReservedAttr as Attribute>::Value,
    ),
    ConfigCls1GpioDirections(<ConfigCls1GpioDirectionsAttr as Attribute>::Value),
    ConfigCls1GpioOutputsDefault(<ConfigCls1GpioOutputsDefaultAttr as Attribute>::Value),
    ConfigCls1GpioTelemetryOnChange(<ConfigCls1GpioTelemetryOnChangeAttr as Attribute>::Value),
    DiagResetCount(<DiagResetCountAttr as Attribute>::Value),
    DiagLastResetReason(<DiagLastResetReasonAttr as Attribute>::Value),
    DiagResetCountUnknown(<DiagResetCountUnknownAttr as Attribute>::Value),
    DiagResetCountPowerOn(<DiagResetCountPowerOnAttr as Attribute>::Value),
    DiagResetCountWatchdog(<DiagResetCountWatchdogAttr as Attribute>::Value),
    DiagResetCountExternal(<DiagResetCountExternalAttr as Attribute>::Value),
    DiagLastRuntimeUptime(<DiagLastRuntimeUptimeAttr as Attribute>::Value),
    DiagLastRuntimeUptimeTotal(<DiagLastRuntimeUptimeTotalAttr as Attribute>::Value),
    DiagLastResetStreakCount(<DiagLastResetStreakCountAttr as Attribute>::Value),
    DiagResetCountBrownOut(<DiagResetCountBrownOutAttr as Attribute>::Value),
    DiagBootSignal(<DiagBootSignalAttr as Attribute>::Value),
}

impl ParsedAttributeValue {
    pub const fn get_meta(&self) -> AttrMeta {
        match self {
            Self::IdNodeId(_) => IdNodeIdAttr::META,
            Self::IdVersion(_) => IdVersionAttr::META,
            Self::IdName(_) => IdNameAttr::META,
            Self::IdMagicNumber(_) => IdMagicNumberAttr::META,
            Self::IdBuildDate(_) => IdBuildDateAttr::META,
            Self::IdBuildCommit(_) => IdBuildCommitAttr::META,
            Self::IdFeatures(_) => IdFeaturesAttr::META,
            Self::SystemUptimeSynced(_) => SystemUptimeSyncedAttr::META,
            Self::SystemTime(_) => SystemTimeAttr::META,
            Self::SystemUptime(_) => SystemUptimeAttr::META,
            Self::SystemStartTime(_) => SystemStartTimeAttr::META,
            Self::SystemLastTelemetry(_) => SystemLastTelemetryAttr::META,
            Self::SystemReceivedTotal(_) => SystemReceivedTotalAttr::META,
            Self::SystemReceivedReadAttribute(_) => SystemReceivedReadAttributeAttr::META,
            Self::SystemReceivedWriteAttribute(_) => SystemReceivedWriteAttributeAttr::META,
            Self::SystemReceivedCommand(_) => SystemReceivedCommandAttr::META,
            Self::SystemReceivedRequestTelemetry(_) => SystemReceivedRequestTelemetryAttr::META,
            Self::SystemReceivedIgnored(_) => SystemReceivedIgnoredAttr::META,
            Self::SystemLastTelemetryMsMod(_) => SystemLastTelemetryMsModAttr::META,
            Self::SystemSentTotal(_) => SystemSentTotalAttr::META,
            Self::SystemSentTelemetry(_) => SystemSentTelemetryAttr::META,
            Self::SystemUnused4(_) => SystemUnused4Attr::META,
            Self::SystemLastCommandError(_) => SystemLastCommandErrorAttr::META,
            Self::SystemLastTelemetryError(_) => SystemLastTelemetryErrorAttr::META,
            Self::SystemUnused5(_) => SystemUnused5Attr::META,
            Self::SystemBattery(_) => SystemBatteryAttr::META,
            Self::ConfigTelemetryPeriod(_) => ConfigTelemetryPeriodAttr::META,
            Self::ConfigTelemetryDelay(_) => ConfigTelemetryDelayAttr::META,
            Self::ConfigTelemetryDelayMin(_) => ConfigTelemetryDelayMinAttr::META,
            Self::ConfigTelemetryDelayMax(_) => ConfigTelemetryDelayMaxAttr::META,
            Self::ConfigFlags(_) => ConfigFlagsAttr::META,
            Self::ConfigTimezone(_) => ConfigTimezoneAttr::META,
            Self::ConfigLocation(_) => ConfigLocationAttr::META,
            Self::ConfigCls0GpioPulseDurationOc1(_) => ConfigCls0GpioPulseDurationOc1Attr::META,
            Self::ConfigCls0GpioPulseDurationOc2(_) => ConfigCls0GpioPulseDurationOc2Attr::META,
            Self::ConfigCls0GpioPulseDurationRl1(_) => ConfigCls0GpioPulseDurationRl1Attr::META,
            Self::ConfigCls0GpioPulseDurationRl2(_) => ConfigCls0GpioPulseDurationRl2Attr::META,
            Self::ConfigCls0GpioOutputsDefault(_) => ConfigCls0GpioOutputsDefaultAttr::META,
            Self::ConfigCls0GpioTelemetryOnChange(_) => ConfigCls0GpioTelemetryOnChangeAttr::META,
            Self::ConfigCls1GpioPulseDurationPc0(_) => ConfigCls1GpioPulseDurationPc0Attr::META,
            Self::ConfigCls1GpioPulseDurationPc1(_) => ConfigCls1GpioPulseDurationPc1Attr::META,
            Self::ConfigCls1GpioPulseDurationPc2(_) => ConfigCls1GpioPulseDurationPc2Attr::META,
            Self::ConfigCls1GpioPulseDurationPc3(_) => ConfigCls1GpioPulseDurationPc3Attr::META,
            Self::ConfigCls1GpioPulseDurationPd0(_) => ConfigCls1GpioPulseDurationPd0Attr::META,
            Self::ConfigCls1GpioPulseDurationPd1(_) => ConfigCls1GpioPulseDurationPd1Attr::META,
            Self::ConfigCls1GpioPulseDurationPd2(_) => ConfigCls1GpioPulseDurationPd2Attr::META,
            Self::ConfigCls1GpioPulseDurationPd3(_) => ConfigCls1GpioPulseDurationPd3Attr::META,
            Self::ConfigCls1GpioPulseDurationPei0(_) => ConfigCls1GpioPulseDurationPei0Attr::META,
            Self::ConfigCls1GpioPulseDurationPei1(_) => ConfigCls1GpioPulseDurationPei1Attr::META,
            Self::ConfigCls1GpioPulseDurationPei2(_) => ConfigCls1GpioPulseDurationPei2Attr::META,
            Self::ConfigCls1GpioPulseDurationPei3(_) => ConfigCls1GpioPulseDurationPei3Attr::META,
            Self::ConfigCls1GpioPulseDurationPei4(_) => ConfigCls1GpioPulseDurationPei4Attr::META,
            Self::ConfigCls1GpioPulseDurationPei5(_) => ConfigCls1GpioPulseDurationPei5Attr::META,
            Self::ConfigCls1GpioPulseDurationPei6(_) => ConfigCls1GpioPulseDurationPei6Attr::META,
            Self::ConfigCls1GpioPulseDurationPei7(_) => ConfigCls1GpioPulseDurationPei7Attr::META,
            Self::ConfigCls1GpioPulseDurationPb0(_) => ConfigCls1GpioPulseDurationPb0Attr::META,
            Self::ConfigCls1GpioPulseDurationPe0(_) => ConfigCls1GpioPulseDurationPe0Attr::META,
            Self::ConfigCls1GpioPulseDurationPe1(_) => ConfigCls1GpioPulseDurationPe1Attr::META,
            Self::ConfigCls1GpioPulseDurationReserved(_) => {
                ConfigCls1GpioPulseDurationReservedAttr::META
            }
            Self::ConfigCls1GpioDirections(_) => ConfigCls1GpioDirectionsAttr::META,
            Self::ConfigCls1GpioOutputsDefault(_) => ConfigCls1GpioOutputsDefaultAttr::META,
            Self::ConfigCls1GpioTelemetryOnChange(_) => ConfigCls1GpioTelemetryOnChangeAttr::META,
            Self::DiagResetCount(_) => DiagResetCountAttr::META,
            Self::DiagLastResetReason(_) => DiagLastResetReasonAttr::META,
            Self::DiagResetCountUnknown(_) => DiagResetCountUnknownAttr::META,
            Self::DiagResetCountPowerOn(_) => DiagResetCountPowerOnAttr::META,
            Self::DiagResetCountWatchdog(_) => DiagResetCountWatchdogAttr::META,
            Self::DiagResetCountExternal(_) => DiagResetCountExternalAttr::META,
            Self::DiagLastRuntimeUptime(_) => DiagLastRuntimeUptimeAttr::META,
            Self::DiagLastRuntimeUptimeTotal(_) => DiagLastRuntimeUptimeTotalAttr::META,
            Self::DiagLastResetStreakCount(_) => DiagLastResetStreakCountAttr::META,
            Self::DiagResetCountBrownOut(_) => DiagResetCountBrownOutAttr::META,
            Self::DiagBootSignal(_) => DiagBootSignalAttr::META,
        }
    }
}

impl fmt::Display for ParsedAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdNodeId(v) => write!(f, "{}", v),
            Self::IdVersion(v) => write!(f, "{}", v),
            Self::IdName(v) => write!(f, "{}", v),
            Self::IdMagicNumber(v) => write!(f, "{}", v),
            Self::IdBuildDate(v) => write!(f, "{}", v),
            Self::IdBuildCommit(v) => write!(f, "{}", v),
            Self::IdFeatures(v) => write!(f, "{}", v),
            Self::SystemUptimeSynced(v) => write!(f, "{}", v),
            Self::SystemTime(v) => write!(f, "{}", v),
            Self::SystemUptime(v) => write!(f, "{}", v),
            Self::SystemStartTime(v) => write!(f, "{}", v),
            Self::SystemLastTelemetry(v) => write!(f, "{}", v),
            Self::SystemReceivedTotal(v) => write!(f, "{}", v),
            Self::SystemReceivedReadAttribute(v) => write!(f, "{}", v),
            Self::SystemReceivedWriteAttribute(v) => write!(f, "{}", v),
            Self::SystemReceivedCommand(v) => write!(f, "{}", v),
            Self::SystemReceivedRequestTelemetry(v) => write!(f, "{}", v),
            Self::SystemReceivedIgnored(v) => write!(f, "{}", v),
            Self::SystemLastTelemetryMsMod(v) => write!(f, "{}", v),
            Self::SystemSentTotal(v) => write!(f, "{}", v),
            Self::SystemSentTelemetry(v) => write!(f, "{}", v),
            Self::SystemUnused4(v) => write!(f, "{}", v),
            Self::SystemLastCommandError(v) => write!(f, "{}", v),
            Self::SystemLastTelemetryError(v) => write!(f, "{}", v),
            Self::SystemUnused5(v) => write!(f, "{}", v),
            Self::SystemBattery(v) => write!(f, "{}", v),
            Self::ConfigTelemetryPeriod(v) => write!(f, "{}", v),
            Self::ConfigTelemetryDelay(v) => write!(f, "{}", v),
            Self::ConfigTelemetryDelayMin(v) => write!(f, "{}", v),
            Self::ConfigTelemetryDelayMax(v) => write!(f, "{}", v),
            Self::ConfigFlags(v) => write!(f, "{}", v),
            Self::ConfigTimezone(v) => write!(f, "{}", v),
            Self::ConfigLocation(v) => write!(f, "{}", v),
            Self::ConfigCls0GpioPulseDurationOc1(v) => write!(f, "{}", v),
            Self::ConfigCls0GpioPulseDurationOc2(v) => write!(f, "{}", v),
            Self::ConfigCls0GpioPulseDurationRl1(v) => write!(f, "{}", v),
            Self::ConfigCls0GpioPulseDurationRl2(v) => write!(f, "{}", v),
            Self::ConfigCls0GpioOutputsDefault(v) => write!(f, "{}", v),
            Self::ConfigCls0GpioTelemetryOnChange(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPc0(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPc1(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPc2(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPc3(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPd0(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPd1(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPd2(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPd3(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei0(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei1(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei2(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei3(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei4(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei5(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei6(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPei7(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPb0(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPe0(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationPe1(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioPulseDurationReserved(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioDirections(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioOutputsDefault(v) => write!(f, "{}", v),
            Self::ConfigCls1GpioTelemetryOnChange(v) => write!(f, "{}", v),
            Self::DiagResetCount(v) => write!(f, "{}", v),
            Self::DiagLastResetReason(v) => write!(f, "{}", v),
            Self::DiagResetCountUnknown(v) => write!(f, "{}", v),
            Self::DiagResetCountPowerOn(v) => write!(f, "{}", v),
            Self::DiagResetCountWatchdog(v) => write!(f, "{}", v),
            Self::DiagResetCountExternal(v) => write!(f, "{}", v),
            Self::DiagLastRuntimeUptime(v) => write!(f, "{}", v),
            Self::DiagLastRuntimeUptimeTotal(v) => write!(f, "{}", v),
            Self::DiagLastResetStreakCount(v) => write!(f, "{}", v),
            Self::DiagResetCountBrownOut(v) => write!(f, "{}", v),
            Self::DiagBootSignal(v) => write!(f, "{}", v),
        }
    }
}

fn all_attr_metas() -> impl Iterator<Item = AttrMeta> {
    [
        IdNodeIdAttr::META,
        IdVersionAttr::META,
        IdNameAttr::META,
        IdMagicNumberAttr::META,
        IdBuildDateAttr::META,
        IdBuildCommitAttr::META,
        IdFeaturesAttr::META,
        SystemUptimeSyncedAttr::META,
        SystemTimeAttr::META,
        SystemUptimeAttr::META,
        SystemStartTimeAttr::META,
        SystemLastTelemetryAttr::META,
        SystemReceivedTotalAttr::META,
        SystemReceivedReadAttributeAttr::META,
        SystemReceivedWriteAttributeAttr::META,
        SystemReceivedCommandAttr::META,
        SystemReceivedRequestTelemetryAttr::META,
        SystemReceivedIgnoredAttr::META,
        SystemLastTelemetryMsModAttr::META,
        SystemSentTotalAttr::META,
        SystemSentTelemetryAttr::META,
        SystemUnused4Attr::META,
        SystemLastCommandErrorAttr::META,
        SystemLastTelemetryErrorAttr::META,
        SystemUnused5Attr::META,
        SystemBatteryAttr::META,
        ConfigTelemetryPeriodAttr::META,
        ConfigTelemetryDelayAttr::META,
        ConfigTelemetryDelayMinAttr::META,
        ConfigTelemetryDelayMaxAttr::META,
        ConfigFlagsAttr::META,
        ConfigTimezoneAttr::META,
        ConfigLocationAttr::META,
        ConfigCls0GpioPulseDurationOc1Attr::META,
        ConfigCls0GpioPulseDurationOc2Attr::META,
        ConfigCls0GpioPulseDurationRl1Attr::META,
        ConfigCls0GpioPulseDurationRl2Attr::META,
        ConfigCls0GpioOutputsDefaultAttr::META,
        ConfigCls0GpioTelemetryOnChangeAttr::META,
        ConfigCls1GpioPulseDurationPc0Attr::META,
        ConfigCls1GpioPulseDurationPc1Attr::META,
        ConfigCls1GpioPulseDurationPc2Attr::META,
        ConfigCls1GpioPulseDurationPc3Attr::META,
        ConfigCls1GpioPulseDurationPd0Attr::META,
        ConfigCls1GpioPulseDurationPd1Attr::META,
        ConfigCls1GpioPulseDurationPd2Attr::META,
        ConfigCls1GpioPulseDurationPd3Attr::META,
        ConfigCls1GpioPulseDurationPei0Attr::META,
        ConfigCls1GpioPulseDurationPei1Attr::META,
        ConfigCls1GpioPulseDurationPei2Attr::META,
        ConfigCls1GpioPulseDurationPei3Attr::META,
        ConfigCls1GpioPulseDurationPei4Attr::META,
        ConfigCls1GpioPulseDurationPei5Attr::META,
        ConfigCls1GpioPulseDurationPei6Attr::META,
        ConfigCls1GpioPulseDurationPei7Attr::META,
        ConfigCls1GpioPulseDurationPb0Attr::META,
        ConfigCls1GpioPulseDurationPe0Attr::META,
        ConfigCls1GpioPulseDurationPe1Attr::META,
        ConfigCls1GpioPulseDurationReservedAttr::META,
        ConfigCls1GpioDirectionsAttr::META,
        ConfigCls1GpioOutputsDefaultAttr::META,
        ConfigCls1GpioTelemetryOnChangeAttr::META,
        DiagResetCountAttr::META,
        DiagLastResetReasonAttr::META,
        DiagResetCountUnknownAttr::META,
        DiagResetCountPowerOnAttr::META,
        DiagResetCountWatchdogAttr::META,
        DiagResetCountExternalAttr::META,
        DiagLastRuntimeUptimeAttr::META,
        DiagLastRuntimeUptimeTotalAttr::META,
        DiagLastResetStreakCountAttr::META,
        DiagResetCountBrownOutAttr::META,
        DiagBootSignalAttr::META,
    ]
    .into_iter()
}

pub fn parse_attr_value(key: u16, raw: u32) -> Option<ParsedAttributeValue> {
    let attr_key = AttrKey::new(key);
    match get_meta_by_key(key)?.id {
        AttrId::IdNodeId => Some(ParsedAttributeValue::IdNodeId(IdNodeIdAttr::parse_value(
            attr_key, raw,
        ))),
        AttrId::IdVersion => Some(ParsedAttributeValue::IdVersion(IdVersionAttr::parse_value(
            attr_key, raw,
        ))),
        AttrId::IdName => Some(ParsedAttributeValue::IdName(IdNameAttr::parse_value(
            attr_key, raw,
        ))),
        AttrId::IdMagicNumber => Some(ParsedAttributeValue::IdMagicNumber(
            IdMagicNumberAttr::parse_value(attr_key, raw),
        )),
        AttrId::IdBuildDate => Some(ParsedAttributeValue::IdBuildDate(
            IdBuildDateAttr::parse_value(attr_key, raw),
        )),
        AttrId::IdBuildCommit => Some(ParsedAttributeValue::IdBuildCommit(
            IdBuildCommitAttr::parse_value(attr_key, raw),
        )),
        AttrId::IdFeatures => Some(ParsedAttributeValue::IdFeatures(
            IdFeaturesAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemUptimeSynced => Some(ParsedAttributeValue::SystemUptimeSynced(
            SystemUptimeSyncedAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemTime => Some(ParsedAttributeValue::SystemTime(
            SystemTimeAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemUptime => Some(ParsedAttributeValue::SystemUptime(
            SystemUptimeAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemStartTime => Some(ParsedAttributeValue::SystemStartTime(
            SystemStartTimeAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemLastTelemetry => Some(ParsedAttributeValue::SystemLastTelemetry(
            SystemLastTelemetryAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemReceivedTotal => Some(ParsedAttributeValue::SystemReceivedTotal(
            SystemReceivedTotalAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemReceivedReadAttribute => {
            Some(ParsedAttributeValue::SystemReceivedReadAttribute(
                SystemReceivedReadAttributeAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::SystemReceivedWriteAttribute => {
            Some(ParsedAttributeValue::SystemReceivedWriteAttribute(
                SystemReceivedWriteAttributeAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::SystemReceivedCommand => Some(ParsedAttributeValue::SystemReceivedCommand(
            SystemReceivedCommandAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemReceivedRequestTelemetry => {
            Some(ParsedAttributeValue::SystemReceivedRequestTelemetry(
                SystemReceivedRequestTelemetryAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::SystemReceivedIgnored => Some(ParsedAttributeValue::SystemReceivedIgnored(
            SystemReceivedIgnoredAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemLastTelemetryMsMod => Some(ParsedAttributeValue::SystemLastTelemetryMsMod(
            SystemLastTelemetryMsModAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemSentTotal => Some(ParsedAttributeValue::SystemSentTotal(
            SystemSentTotalAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemSentTelemetry => Some(ParsedAttributeValue::SystemSentTelemetry(
            SystemSentTelemetryAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemUnused4 => Some(ParsedAttributeValue::SystemUnused4(
            SystemUnused4Attr::parse_value(attr_key, raw),
        )),
        AttrId::SystemLastCommandError => Some(ParsedAttributeValue::SystemLastCommandError(
            SystemLastCommandErrorAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemLastTelemetryError => Some(ParsedAttributeValue::SystemLastTelemetryError(
            SystemLastTelemetryErrorAttr::parse_value(attr_key, raw),
        )),
        AttrId::SystemUnused5 => Some(ParsedAttributeValue::SystemUnused5(
            SystemUnused5Attr::parse_value(attr_key, raw),
        )),
        AttrId::SystemBattery => Some(ParsedAttributeValue::SystemBattery(
            SystemBatteryAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigTelemetryPeriod => Some(ParsedAttributeValue::ConfigTelemetryPeriod(
            ConfigTelemetryPeriodAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigTelemetryDelay => Some(ParsedAttributeValue::ConfigTelemetryDelay(
            ConfigTelemetryDelayAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigTelemetryDelayMin => Some(ParsedAttributeValue::ConfigTelemetryDelayMin(
            ConfigTelemetryDelayMinAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigTelemetryDelayMax => Some(ParsedAttributeValue::ConfigTelemetryDelayMax(
            ConfigTelemetryDelayMaxAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigFlags => Some(ParsedAttributeValue::ConfigFlags(
            ConfigFlagsAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigTimezone => Some(ParsedAttributeValue::ConfigTimezone(
            ConfigTimezoneAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigLocation => Some(ParsedAttributeValue::ConfigLocation(
            ConfigLocationAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigCls0GpioPulseDurationOc1 => {
            Some(ParsedAttributeValue::ConfigCls0GpioPulseDurationOc1(
                ConfigCls0GpioPulseDurationOc1Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls0GpioPulseDurationOc2 => {
            Some(ParsedAttributeValue::ConfigCls0GpioPulseDurationOc2(
                ConfigCls0GpioPulseDurationOc2Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls0GpioPulseDurationRl1 => {
            Some(ParsedAttributeValue::ConfigCls0GpioPulseDurationRl1(
                ConfigCls0GpioPulseDurationRl1Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls0GpioPulseDurationRl2 => {
            Some(ParsedAttributeValue::ConfigCls0GpioPulseDurationRl2(
                ConfigCls0GpioPulseDurationRl2Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls0GpioOutputsDefault => {
            Some(ParsedAttributeValue::ConfigCls0GpioOutputsDefault(
                ConfigCls0GpioOutputsDefaultAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls0GpioTelemetryOnChange => {
            Some(ParsedAttributeValue::ConfigCls0GpioTelemetryOnChange(
                ConfigCls0GpioTelemetryOnChangeAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPc0 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPc0(
                ConfigCls1GpioPulseDurationPc0Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPc1 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPc1(
                ConfigCls1GpioPulseDurationPc1Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPc2 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPc2(
                ConfigCls1GpioPulseDurationPc2Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPc3 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPc3(
                ConfigCls1GpioPulseDurationPc3Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPd0 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPd0(
                ConfigCls1GpioPulseDurationPd0Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPd1 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPd1(
                ConfigCls1GpioPulseDurationPd1Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPd2 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPd2(
                ConfigCls1GpioPulseDurationPd2Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPd3 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPd3(
                ConfigCls1GpioPulseDurationPd3Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei0 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei0(
                ConfigCls1GpioPulseDurationPei0Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei1 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei1(
                ConfigCls1GpioPulseDurationPei1Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei2 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei2(
                ConfigCls1GpioPulseDurationPei2Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei3 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei3(
                ConfigCls1GpioPulseDurationPei3Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei4 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei4(
                ConfigCls1GpioPulseDurationPei4Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei5 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei5(
                ConfigCls1GpioPulseDurationPei5Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei6 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei6(
                ConfigCls1GpioPulseDurationPei6Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPei7 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPei7(
                ConfigCls1GpioPulseDurationPei7Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPb0 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPb0(
                ConfigCls1GpioPulseDurationPb0Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPe0 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPe0(
                ConfigCls1GpioPulseDurationPe0Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationPe1 => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationPe1(
                ConfigCls1GpioPulseDurationPe1Attr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioPulseDurationReserved => {
            Some(ParsedAttributeValue::ConfigCls1GpioPulseDurationReserved(
                ConfigCls1GpioPulseDurationReservedAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioDirections => Some(ParsedAttributeValue::ConfigCls1GpioDirections(
            ConfigCls1GpioDirectionsAttr::parse_value(attr_key, raw),
        )),
        AttrId::ConfigCls1GpioOutputsDefault => {
            Some(ParsedAttributeValue::ConfigCls1GpioOutputsDefault(
                ConfigCls1GpioOutputsDefaultAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::ConfigCls1GpioTelemetryOnChange => {
            Some(ParsedAttributeValue::ConfigCls1GpioTelemetryOnChange(
                ConfigCls1GpioTelemetryOnChangeAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::DiagResetCount => Some(ParsedAttributeValue::DiagResetCount(
            DiagResetCountAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagLastResetReason => Some(ParsedAttributeValue::DiagLastResetReason(
            DiagLastResetReasonAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagResetCountUnknown => Some(ParsedAttributeValue::DiagResetCountUnknown(
            DiagResetCountUnknownAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagResetCountPowerOn => Some(ParsedAttributeValue::DiagResetCountPowerOn(
            DiagResetCountPowerOnAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagResetCountWatchdog => Some(ParsedAttributeValue::DiagResetCountWatchdog(
            DiagResetCountWatchdogAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagResetCountExternal => Some(ParsedAttributeValue::DiagResetCountExternal(
            DiagResetCountExternalAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagLastRuntimeUptime => Some(ParsedAttributeValue::DiagLastRuntimeUptime(
            DiagLastRuntimeUptimeAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagLastRuntimeUptimeTotal => {
            Some(ParsedAttributeValue::DiagLastRuntimeUptimeTotal(
                DiagLastRuntimeUptimeTotalAttr::parse_value(attr_key, raw),
            ))
        }
        AttrId::DiagLastResetStreakCount => Some(ParsedAttributeValue::DiagLastResetStreakCount(
            DiagLastResetStreakCountAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagResetCountBrownOut => Some(ParsedAttributeValue::DiagResetCountBrownOut(
            DiagResetCountBrownOutAttr::parse_value(attr_key, raw),
        )),
        AttrId::DiagBootSignal => Some(ParsedAttributeValue::DiagBootSignal(
            DiagBootSignalAttr::parse_value(attr_key, raw),
        )),
    }
}
pub fn get_meta_by_key(key: u16) -> Option<AttrMeta> {
    let key = AttrKey::new(key);

    all_attr_metas().find(|attr| attr.contains_key(key))
}

pub fn get_meta_by_display_name(display_name: &str) -> Option<AttrMeta> {
    all_attr_metas().find(|attr| attr.display_name.eq_ignore_ascii_case(display_name))
}

pub fn iter_attrs() -> impl Iterator<Item = AttrMeta> {
    all_attr_metas()
}
