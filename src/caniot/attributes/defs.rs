use super::core::{AttrId, AttrKey, AttrMeta, Attribute, Section};
use super::values::*;

pub struct IdNodeIdAttr;

impl Attribute for IdNodeIdAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdNodeId,
        display_name: "Device ID",
        size: 1,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = DeviceIdValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_device_id(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdVersionAttr;

impl Attribute for IdVersionAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdVersion,
        display_name: "Firmware version",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = VersionValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_version(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdNameAttr;

impl Attribute for IdNameAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdName,
        display_name: "Device name",
        size: 32,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = TextPartValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_text(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdMagicNumberAttr;

impl Attribute for IdMagicNumberAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdMagicNumber,
        display_name: "Device magic number",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = Hex32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_hex32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdBuildDateAttr;

impl Attribute for IdBuildDateAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdBuildDate,
        display_name: "Firmware build date",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = TimestampValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_timestamp(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdBuildCommitAttr;

impl Attribute for IdBuildCommitAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdBuildCommit,
        display_name: "Firmware build commit",
        size: 20,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = BytesPartValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_bytes_part(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdFeaturesAttr;

impl Attribute for IdFeaturesAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::IdFeatures,
        display_name: "Firmware features",
        size: 16,
        read: true,
        write: false,
        persistent: false,
        section: Section::Identification,
    };
    type Value = BytesPartValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_bytes_part(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemUptimeSyncedAttr;

impl Attribute for SystemUptimeSyncedAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemUptimeSynced,
        display_name: "Last time synchronization uptime",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = SecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_seconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemTimeAttr;

impl Attribute for SystemTimeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemTime,
        display_name: "Current time",
        size: 4,
        read: true,
        write: true,
        persistent: false,
        section: Section::System,
    };
    type Value = TimestampValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_timestamp(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemUptimeAttr;

impl Attribute for SystemUptimeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemUptime,
        display_name: "System uptime",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = SecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_seconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemStartTimeAttr;

impl Attribute for SystemStartTimeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemStartTime,
        display_name: "System start time",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = TimestampValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_timestamp(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemLastTelemetryAttr;

impl Attribute for SystemLastTelemetryAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemLastTelemetry,
        display_name: "Last telemetry time",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = TimestampValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_timestamp(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemReceivedTotalAttr;

impl Attribute for SystemReceivedTotalAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemReceivedTotal,
        display_name: "Received frames",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemReceivedReadAttributeAttr;

impl Attribute for SystemReceivedReadAttributeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemReceivedReadAttribute,
        display_name: "Received read-attribute requests",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemReceivedWriteAttributeAttr;

impl Attribute for SystemReceivedWriteAttributeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemReceivedWriteAttribute,
        display_name: "Received write-attribute requests",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemReceivedCommandAttr;

impl Attribute for SystemReceivedCommandAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemReceivedCommand,
        display_name: "Received commands",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemReceivedRequestTelemetryAttr;

impl Attribute for SystemReceivedRequestTelemetryAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemReceivedRequestTelemetry,
        display_name: "Received telemetry requests",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemReceivedIgnoredAttr;

impl Attribute for SystemReceivedIgnoredAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemReceivedIgnored,
        display_name: "Ignored received frames",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemLastTelemetryMsModAttr;

impl Attribute for SystemLastTelemetryMsModAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemLastTelemetryMsMod,
        display_name: "Last telemetry millisecond counter",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemSentTotalAttr;

impl Attribute for SystemSentTotalAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemSentTotal,
        display_name: "Sent frames",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemSentTelemetryAttr;

impl Attribute for SystemSentTelemetryAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemSentTelemetry,
        display_name: "Sent telemetry frames",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemUnused4Attr;

impl Attribute for SystemUnused4Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemUnused4,
        display_name: "Reserved system field 4",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemLastCommandErrorAttr;

impl Attribute for SystemLastCommandErrorAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemLastCommandError,
        display_name: "Last command error",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = I16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_i16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemLastTelemetryErrorAttr;

impl Attribute for SystemLastTelemetryErrorAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemLastTelemetryError,
        display_name: "Last telemetry error",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = I16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_i16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemUnused5Attr;

impl Attribute for SystemUnused5Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemUnused5,
        display_name: "Reserved system field 5",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = I16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_i16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemBatteryAttr;

impl Attribute for SystemBatteryAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::SystemBattery,
        display_name: "Battery level",
        size: 1,
        read: true,
        write: false,
        persistent: false,
        section: Section::System,
    };
    type Value = U8Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u8(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigTelemetryPeriodAttr;

impl Attribute for ConfigTelemetryPeriodAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigTelemetryPeriod,
        display_name: "Telemetry period",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigTelemetryDelayAttr;

impl Attribute for ConfigTelemetryDelayAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigTelemetryDelay,
        display_name: "Telemetry delay",
        size: 2,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigTelemetryDelayMinAttr;

impl Attribute for ConfigTelemetryDelayMinAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigTelemetryDelayMin,
        display_name: "Minimum telemetry delay",
        size: 2,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigTelemetryDelayMaxAttr;

impl Attribute for ConfigTelemetryDelayMaxAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigTelemetryDelayMax,
        display_name: "Maximum telemetry delay",
        size: 2,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigFlagsAttr;

impl Attribute for ConfigFlagsAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigFlags,
        display_name: "Configuration flags",
        size: 1,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = ConfigFlagsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_config_flags(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigTimezoneAttr;

impl Attribute for ConfigTimezoneAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigTimezone,
        display_name: "Device timezone",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = TimezoneValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_timezone(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigLocationAttr;

impl Attribute for ConfigLocationAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigLocation,
        display_name: "Device location",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = LocationValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_location(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls0GpioPulseDurationOc1Attr;

impl Attribute for ConfigCls0GpioPulseDurationOc1Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls0GpioPulseDurationOc1,
        display_name: "Class 0 OC1 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls0GpioPulseDurationOc2Attr;

impl Attribute for ConfigCls0GpioPulseDurationOc2Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls0GpioPulseDurationOc2,
        display_name: "Class 0 OC2 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls0GpioPulseDurationRl1Attr;

impl Attribute for ConfigCls0GpioPulseDurationRl1Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls0GpioPulseDurationRl1,
        display_name: "Class 0 relay 1 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls0GpioPulseDurationRl2Attr;

impl Attribute for ConfigCls0GpioPulseDurationRl2Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls0GpioPulseDurationRl2,
        display_name: "Class 0 relay 2 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls0GpioOutputsDefaultAttr;

impl Attribute for ConfigCls0GpioOutputsDefaultAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls0GpioOutputsDefault,
        display_name: "Class 0 default output states",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = Hex32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_hex32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls0GpioTelemetryOnChangeAttr;

impl Attribute for ConfigCls0GpioTelemetryOnChangeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls0GpioTelemetryOnChange,
        display_name: "Class 0 telemetry-on-change mask",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = Hex32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_hex32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPc0Attr;

impl Attribute for ConfigCls1GpioPulseDurationPc0Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPc0,
        display_name: "Class 1 PC0 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPc1Attr;

impl Attribute for ConfigCls1GpioPulseDurationPc1Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPc1,
        display_name: "Class 1 PC1 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPc2Attr;

impl Attribute for ConfigCls1GpioPulseDurationPc2Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPc2,
        display_name: "Class 1 PC2 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPc3Attr;

impl Attribute for ConfigCls1GpioPulseDurationPc3Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPc3,
        display_name: "Class 1 PC3 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPd0Attr;

impl Attribute for ConfigCls1GpioPulseDurationPd0Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPd0,
        display_name: "Class 1 PD0 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPd1Attr;

impl Attribute for ConfigCls1GpioPulseDurationPd1Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPd1,
        display_name: "Class 1 PD1 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPd2Attr;

impl Attribute for ConfigCls1GpioPulseDurationPd2Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPd2,
        display_name: "Class 1 PD2 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPd3Attr;

impl Attribute for ConfigCls1GpioPulseDurationPd3Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPd3,
        display_name: "Class 1 PD3 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei0Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei0Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei0,
        display_name: "Class 1 PEI0 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei1Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei1Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei1,
        display_name: "Class 1 PEI1 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei2Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei2Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei2,
        display_name: "Class 1 PEI2 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei3Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei3Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei3,
        display_name: "Class 1 PEI3 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei4Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei4Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei4,
        display_name: "Class 1 PEI4 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei5Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei5Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei5,
        display_name: "Class 1 PEI5 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei6Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei6Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei6,
        display_name: "Class 1 PEI6 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPei7Attr;

impl Attribute for ConfigCls1GpioPulseDurationPei7Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPei7,
        display_name: "Class 1 PEI7 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPb0Attr;

impl Attribute for ConfigCls1GpioPulseDurationPb0Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPb0,
        display_name: "Class 1 PB0 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPe0Attr;

impl Attribute for ConfigCls1GpioPulseDurationPe0Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPe0,
        display_name: "Class 1 PE0 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationPe1Attr;

impl Attribute for ConfigCls1GpioPulseDurationPe1Attr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationPe1,
        display_name: "Class 1 PE1 pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioPulseDurationReservedAttr;

impl Attribute for ConfigCls1GpioPulseDurationReservedAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioPulseDurationReserved,
        display_name: "Class 1 reserved pulse duration",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = MillisecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_milliseconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioDirectionsAttr;

impl Attribute for ConfigCls1GpioDirectionsAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioDirections,
        display_name: "Class 1 GPIO directions",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = Hex32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_hex32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioOutputsDefaultAttr;

impl Attribute for ConfigCls1GpioOutputsDefaultAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioOutputsDefault,
        display_name: "Class 1 default output states",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = Hex32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_hex32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigCls1GpioTelemetryOnChangeAttr;

impl Attribute for ConfigCls1GpioTelemetryOnChangeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::ConfigCls1GpioTelemetryOnChange,
        display_name: "Class 1 telemetry-on-change mask",
        size: 4,
        read: true,
        write: true,
        persistent: true,
        section: Section::Config,
    };
    type Value = Hex32Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_hex32(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagResetCountAttr;

impl Attribute for DiagResetCountAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagResetCount,
        display_name: "Reset count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagLastResetReasonAttr;

impl Attribute for DiagLastResetReasonAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagLastResetReason,
        display_name: "Last reset reason",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = ResetReasonValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_reset_reason(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagResetCountUnknownAttr;

impl Attribute for DiagResetCountUnknownAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagResetCountUnknown,
        display_name: "Unknown reset count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagResetCountPowerOnAttr;

impl Attribute for DiagResetCountPowerOnAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagResetCountPowerOn,
        display_name: "Power-on reset count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagResetCountWatchdogAttr;

impl Attribute for DiagResetCountWatchdogAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagResetCountWatchdog,
        display_name: "Watchdog reset count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagResetCountExternalAttr;

impl Attribute for DiagResetCountExternalAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagResetCountExternal,
        display_name: "External reset count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagLastRuntimeUptimeAttr;

impl Attribute for DiagLastRuntimeUptimeAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagLastRuntimeUptime,
        display_name: "Last runtime uptime",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = SecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_seconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagLastRuntimeUptimeTotalAttr;

impl Attribute for DiagLastRuntimeUptimeTotalAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagLastRuntimeUptimeTotal,
        display_name: "Total runtime uptime",
        size: 4,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = SecondsValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_seconds(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagLastResetStreakCountAttr;

impl Attribute for DiagLastResetStreakCountAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagLastResetStreakCount,
        display_name: "Reset streak count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagResetCountBrownOutAttr;

impl Attribute for DiagResetCountBrownOutAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagResetCountBrownOut,
        display_name: "Brown-out reset count",
        size: 2,
        read: true,
        write: false,
        persistent: false,
        section: Section::Diag,
    };
    type Value = U16Value;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_u16(key, raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagBootSignalAttr;

impl Attribute for DiagBootSignalAttr {
    const META: AttrMeta = AttrMeta {
        id: AttrId::DiagBootSignal,
        display_name: "Boot signal",
        size: 4,
        read: true,
        write: true,
        persistent: false,
        section: Section::Diag,
    };
    type Value = PartialCommitHashValue;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value {
        parse_partial_commit_hash(key, raw)
    }
}
