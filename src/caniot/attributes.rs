use chrono::{DateTime, Utc};
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Section {
    Identification = 0,
    System = 1,
    Config = 2,
    Diag = 3,
    Unknown = 0xff,
}

impl From<u8> for Section {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Identification,
            1 => Self::System,
            2 => Self::Config,
            3 => Self::Diag,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Identification => "Identification",
            Self::System => "System",
            Self::Config => "Configuration",
            Self::Diag => "Diagnostics",
            Self::Unknown => "Unknown",
        };

        f.write_str(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttrKey {
    pub raw: u16,
    pub section: Section,
    pub attr: u8,
    pub part: u8,
}

impl AttrKey {
    pub const fn new(raw: u16) -> Self {
        Self {
            raw,
            section: match ((raw >> 12) & 0x0f) as u8 {
                0 => Section::Identification,
                1 => Section::System,
                2 => Section::Config,
                3 => Section::Diag,
                _ => Section::Unknown,
            },
            attr: ((raw >> 4) & 0xff) as u8,
            part: (raw & 0x0f) as u8,
        }
    }

    pub const fn root(self) -> u16 {
        self.raw & 0xfff0
    }
}

impl fmt::Display for AttrKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{:04X} ({}/{}/{})",
            self.raw, self.section, self.attr, self.part
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum AttrId {
    IdNodeId = 0x0000,
    IdVersion = 0x0010,
    IdName = 0x0020,
    IdMagicNumber = 0x0030,
    IdBuildDate = 0x0040,
    IdBuildCommit = 0x0050,
    IdFeatures = 0x0060,

    SystemUptimeSynced = 0x1000,
    SystemTime = 0x1010,
    SystemUptime = 0x1020,
    SystemStartTime = 0x1030,
    SystemLastTelemetry = 0x1040,
    SystemReceivedTotal = 0x1050,
    SystemReceivedReadAttribute = 0x1060,
    SystemReceivedWriteAttribute = 0x1070,
    SystemReceivedCommand = 0x1080,
    SystemReceivedRequestTelemetry = 0x1090,
    SystemReceivedIgnored = 0x10A0,
    SystemLastTelemetryMsMod = 0x10B0,
    SystemSentTotal = 0x10C0,
    SystemSentTelemetry = 0x10D0,
    SystemUnused4 = 0x10E0,
    SystemLastCommandError = 0x10F0,
    SystemLastTelemetryError = 0x1100,
    SystemUnused5 = 0x1110,
    SystemBattery = 0x1120,

    ConfigTelemetryPeriod = 0x2000,
    ConfigTelemetryDelay = 0x2010,
    ConfigTelemetryDelayMin = 0x2020,
    ConfigTelemetryDelayMax = 0x2030,
    ConfigFlags = 0x2040,
    ConfigTimezone = 0x2050,
    ConfigLocation = 0x2060,

    ConfigCls0GpioPulseDurationOc1 = 0x2070,
    ConfigCls0GpioPulseDurationOc2 = 0x2080,
    ConfigCls0GpioPulseDurationRl1 = 0x2090,
    ConfigCls0GpioPulseDurationRl2 = 0x20A0,
    ConfigCls0GpioOutputsDefault = 0x20B0,
    ConfigCls0GpioTelemetryOnChange = 0x20C0,

    ConfigCls1GpioPulseDurationPc0 = 0x20D0,
    ConfigCls1GpioPulseDurationPc1 = 0x20E0,
    ConfigCls1GpioPulseDurationPc2 = 0x20F0,
    ConfigCls1GpioPulseDurationPc3 = 0x2100,
    ConfigCls1GpioPulseDurationPd0 = 0x2110,
    ConfigCls1GpioPulseDurationPd1 = 0x2120,
    ConfigCls1GpioPulseDurationPd2 = 0x2130,
    ConfigCls1GpioPulseDurationPd3 = 0x2140,
    ConfigCls1GpioPulseDurationPei0 = 0x2150,
    ConfigCls1GpioPulseDurationPei1 = 0x2160,
    ConfigCls1GpioPulseDurationPei2 = 0x2170,
    ConfigCls1GpioPulseDurationPei3 = 0x2180,
    ConfigCls1GpioPulseDurationPei4 = 0x2190,
    ConfigCls1GpioPulseDurationPei5 = 0x21A0,
    ConfigCls1GpioPulseDurationPei6 = 0x21B0,
    ConfigCls1GpioPulseDurationPei7 = 0x21C0,
    ConfigCls1GpioPulseDurationPb0 = 0x21D0,
    ConfigCls1GpioPulseDurationPe0 = 0x21E0,
    ConfigCls1GpioPulseDurationPe1 = 0x21F0,
    ConfigCls1GpioPulseDurationReserved = 0x2200,
    ConfigCls1GpioDirections = 0x2210,
    ConfigCls1GpioOutputsDefault = 0x2220,
    ConfigCls1GpioTelemetryOnChange = 0x2230,

    DiagResetCount = 0x3000,
    DiagLastResetReason = 0x3010,
    DiagResetCountUnknown = 0x3020,
    DiagResetCountPowerOn = 0x3030,
    DiagResetCountWatchdog = 0x3040,
    DiagResetCountExternal = 0x3050,
    DiagLastRuntimeUptime = 0x3060,
    DiagLastRuntimeUptimeTotal = 0x3070,
    DiagLastResetStreakCount = 0x3080,
    DiagResetCountBrownOut = 0x3090,
    DiagBootSignal = 0x3100,
}

impl AttrId {
    pub const fn key(self) -> u16 {
        self as u16
    }

    pub const fn attr_key(self) -> AttrKey {
        AttrKey::new(self.key())
    }

    pub fn meta(self) -> Option<AttrMeta> {
        get_meta_by_key(self.key())
    }

    pub fn parse(self, raw: u32) -> Option<Attribute> {
        parse_attr(self.key(), raw)
    }
}

impl From<AttrId> for u16 {
    fn from(id: AttrId) -> Self {
        id.key()
    }
}

impl From<AttrId> for AttrKey {
    fn from(id: AttrId) -> Self {
        id.attr_key()
    }
}

impl fmt::Display for AttrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = self.key();

        if let Some(meta) = get_meta_by_key(key) {
            write!(f, "{} / 0x{:04X}", meta.display_name, key)
        } else {
            write!(f, "0x{:04X}", key)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrKind {
    U8,
    U16,
    I16,
    U32,
    I32,
    DeviceId,
    Version,
    Text,
    Timestamp,
    Seconds,
    Milliseconds,
    ConfigFlags,
    Timezone,
    Location,
    ResetReason,
    Bytes,
    Hex32,
    PartialCommitHash,
}

#[derive(Debug, Clone, Copy)]
pub struct AttrMeta {
    pub id: AttrId,
    pub display_name: &'static str,
    pub size: u8,
    pub read: bool,
    pub write: bool,
    pub persistent: bool,
    pub section: Section,
    pub kind: AttrKind,
}

impl AttrMeta {
    pub const fn key(self) -> u16 {
        self.id.key()
    }

    pub const fn parts(self) -> u8 {
        (self.size + 3) / 4
    }

    pub fn contains_key(self, key: AttrKey) -> bool {
        let my = self.id.attr_key();

        my.section as u8 == key.section as u8 && my.attr == key.attr && key.part < self.parts()
    }

    pub fn part_key(self, part: u8) -> Option<u16> {
        if part < self.parts() {
            Some(self.key() + part as u16)
        } else {
            None
        }
    }

    pub fn parse(self, key: u16, raw: u32) -> Option<Attribute> {
        let key = AttrKey::new(key);

        if !self.contains_key(key) {
            return None;
        }

        Some(Attribute {
            meta: self,
            key,
            raw,
            value: AttrValue::parse(self.kind, key, raw),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttrValue {
    U8(u8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),

    DeviceId {
        raw: u8,
        class: u8,
        sid: u8,
    },

    Version {
        caniot: u8,
        application: u8,
    },

    TextPart {
        part: u8,
        text: String,
    },

    Timestamp(u32),
    Seconds(u32),
    Milliseconds(u32),

    ConfigFlags {
        error_response: bool,
        telemetry_delay_random: bool,
        telemetry_endpoint: u8,
        telemetry_periodic_enabled: bool,
    },

    Timezone(i32),

    Location {
        region: String,
        country: String,
    },

    ResetReason(&'static str),

    BytesPart {
        part: u8,
        bytes: Vec<u8>,
    },

    Hex32(u32),
    PartialCommitHash {
        raw: u32,
        bytes: [u8; 4],
    },
}

impl AttrValue {
    pub fn parse(kind: AttrKind, key: AttrKey, raw: u32) -> Self {
        match kind {
            AttrKind::U8 => Self::U8(raw as u8),
            AttrKind::U16 => Self::U16(raw as u16),
            AttrKind::I16 => Self::I16(i16::from_le_bytes((raw as u16).to_le_bytes())),
            AttrKind::U32 => Self::U32(raw),
            AttrKind::I32 => Self::I32(i32::from_le_bytes(raw.to_le_bytes())),
            AttrKind::DeviceId => {
                let did = raw as u8;

                Self::DeviceId {
                    raw: did,
                    class: (did >> 3) & 0x07,
                    sid: did & 0x07,
                }
            }
            AttrKind::Version => Self::Version {
                caniot: ((raw >> 8) & 0xff) as u8,
                application: (raw & 0xff) as u8,
            },
            AttrKind::Text => {
                let bytes = raw.to_le_bytes();

                Self::TextPart {
                    part: key.part,
                    text: String::from_utf8_lossy(&bytes)
                        .trim_end_matches('\0')
                        .to_string(),
                }
            }
            AttrKind::Timestamp => Self::Timestamp(raw),
            AttrKind::Seconds => Self::Seconds(raw),
            AttrKind::Milliseconds => Self::Milliseconds(raw),
            AttrKind::ConfigFlags => Self::ConfigFlags {
                error_response: raw & (1 << 0) != 0,
                telemetry_delay_random: raw & (1 << 1) != 0,
                telemetry_endpoint: ((raw >> 2) & 0x03) as u8,
                telemetry_periodic_enabled: raw & (1 << 4) != 0,
            },

            AttrKind::Timezone => Self::Timezone(i32::from_le_bytes(raw.to_le_bytes())),
            AttrKind::Location => {
                let b = raw.to_le_bytes();

                Self::Location {
                    region: String::from_utf8_lossy(&b[0..2]).to_string(),
                    country: String::from_utf8_lossy(&b[2..4]).to_string(),
                }
            }
            AttrKind::ResetReason => {
                let reason = match raw {
                    0 => "unknown",
                    1 => "power-on",
                    2 => "watchdog",
                    3 => "external",
                    4 => "brown-out",
                    _ => "invalid",
                };

                Self::ResetReason(reason)
            }
            AttrKind::Bytes => Self::BytesPart {
                part: key.part,
                bytes: raw.to_le_bytes().to_vec(),
            },
            AttrKind::Hex32 => Self::Hex32(raw),
            AttrKind::PartialCommitHash => Self::PartialCommitHash {
                raw,
                bytes: raw.to_le_bytes(),
            },
        }
    }
}

impl fmt::Display for AttrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{} / 0x{:02X}", v, v),
            Self::U16(v) => write!(f, "{} / 0x{:04X}", v, v),
            Self::I16(v) => write!(f, "{}", v),
            Self::U32(v) => write!(f, "{} / 0x{:08X}", v, v),
            Self::I32(v) => write!(f, "{}", v),
            Self::DeviceId { raw, class, sid } => {
                write!(f, "0x{:02X} (class {}, sid {})", raw, class, sid)
            }
            Self::Version { caniot, application } => {
                write!(f, "CANIOT {}, firmware {}", caniot, application)
            }
            Self::TextPart { part, text } => {
                write!(f, "part {}: {:?}", part, text)
            }
            Self::Timestamp(ts) => {
                if *ts == 0 {
                    write!(f, "not set")
                } else if let Some(dt) = DateTime::<Utc>::from_timestamp(*ts as i64, 0) {
                    write!(f, "{} UTC", dt.format("%Y-%m-%d %H:%M:%S"))
                } else {
                    write!(f, "{} epoch-s", ts)
                }
            }
            Self::Seconds(v) => {
                let days = v / 86_400;
                let hours = (v % 86_400) / 3_600;
                let minutes = (v % 3_600) / 60;
                let seconds = v % 60;

                if days > 0 {
                    write!(f, "{}d {}h {}m {}s", days, hours, minutes, seconds)
                } else {
                    write!(f, "{}h {}m {}s", hours, minutes, seconds)
                }
            }
            Self::Milliseconds(v) => write!(f, "{} ms", v),
            Self::ConfigFlags {
                error_response,
                telemetry_delay_random,
                telemetry_endpoint,
                telemetry_periodic_enabled,
            } => write!(
                f,
                "error_response={}, telemetry_delay_random={}, telemetry_endpoint={}, telemetry_periodic_enabled={}",
                *error_response as u8,
                *telemetry_delay_random as u8,
                telemetry_endpoint,
                *telemetry_periodic_enabled as u8
            ),
            Self::Timezone(seconds) => {
                let sign = if *seconds >= 0 { '+' } else { '-' };
                let abs = seconds.unsigned_abs();
                let h = abs / 3_600;
                let m = (abs % 3_600) / 60;

                write!(f, "{}{:02}:{:02}", sign, h, m)
            }
            Self::Location { region, country } => {
                write!(f, "{}/{}", region, country)
            }

            Self::ResetReason(reason) => f.write_str(reason),
            Self::BytesPart { part, bytes } => {
                write!(f, "part {}: ", part)?;

                for b in bytes {
                    write!(f, "{:02x}", b)?;
                }

                Ok(())
            }
            Self::Hex32(v) => write!(f, "0x{:08X}", v),
            Self::PartialCommitHash { bytes, .. } => {
                for b in bytes {
                    write!(f, "{:02x}", b)?;
                }

                f.write_str("...")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub meta: AttrMeta,
    pub key: AttrKey,
    pub raw: u32,
    pub value: AttrValue,
}

impl Attribute {
    pub fn display_name(&self) -> &'static str {
        self.meta.display_name
    }

    pub fn is_readable(&self) -> bool {
        self.meta.read
    }

    pub fn is_writable(&self) -> bool {
        self.meta.write
    }

    pub fn is_persistent(&self) -> bool {
        self.meta.persistent
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let access = match (self.meta.read, self.meta.write) {
            (true, true) => "rw",
            (true, false) => "ro",
            (false, true) => "wo",
            (false, false) => "--",
        };

        write!(
            f,
            "{} [{}] = {}",
            self.meta.display_name, access, self.value
        )
    }
}

macro_rules! attr {
    ($id:ident, $display_name:expr, $size:expr, $read:expr, $write:expr, $persistent:expr, $section:expr, $kind:expr) => {
        AttrMeta {
            id: AttrId::$id,
            display_name: $display_name,
            size: $size,
            read: $read,
            write: $write,
            persistent: $persistent,
            section: $section,
            kind: $kind,
        }
    };
}

pub static ATTRS: &[AttrMeta] = &[
    attr!(
        IdNodeId,
        "Device ID",
        1,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::DeviceId
    ),
    attr!(
        IdVersion,
        "Firmware version",
        2,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::Version
    ),
    attr!(
        IdName,
        "Device name",
        32,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::Text
    ),
    attr!(
        IdMagicNumber,
        "Device magic number",
        4,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::Hex32
    ),
    attr!(
        IdBuildDate,
        "Firmware build date",
        4,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::Timestamp
    ),
    attr!(
        IdBuildCommit,
        "Firmware build commit",
        20,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::Bytes
    ),
    attr!(
        IdFeatures,
        "Firmware features",
        16,
        true,
        false,
        false,
        Section::Identification,
        AttrKind::Bytes
    ),
    attr!(
        SystemUptimeSynced,
        "Last time synchronization uptime",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::Seconds
    ),
    attr!(
        SystemTime,
        "Current time",
        4,
        true,
        true,
        false,
        Section::System,
        AttrKind::Timestamp
    ),
    attr!(
        SystemUptime,
        "System uptime",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::Seconds
    ),
    attr!(
        SystemStartTime,
        "System start time",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::Timestamp
    ),
    attr!(
        SystemLastTelemetry,
        "Last telemetry time",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::Timestamp
    ),
    attr!(
        SystemReceivedTotal,
        "Received frames",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemReceivedReadAttribute,
        "Received read-attribute requests",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemReceivedWriteAttribute,
        "Received write-attribute requests",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemReceivedCommand,
        "Received commands",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemReceivedRequestTelemetry,
        "Received telemetry requests",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemReceivedIgnored,
        "Ignored received frames",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemLastTelemetryMsMod,
        "Last telemetry millisecond counter",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::Milliseconds
    ),
    attr!(
        SystemSentTotal,
        "Sent frames",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemSentTelemetry,
        "Sent telemetry frames",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemUnused4,
        "Reserved system field 4",
        4,
        true,
        false,
        false,
        Section::System,
        AttrKind::U32
    ),
    attr!(
        SystemLastCommandError,
        "Last command error",
        2,
        true,
        false,
        false,
        Section::System,
        AttrKind::I16
    ),
    attr!(
        SystemLastTelemetryError,
        "Last telemetry error",
        2,
        true,
        false,
        false,
        Section::System,
        AttrKind::I16
    ),
    attr!(
        SystemUnused5,
        "Reserved system field 5",
        2,
        true,
        false,
        false,
        Section::System,
        AttrKind::I16
    ),
    attr!(
        SystemBattery,
        "Battery level",
        1,
        true,
        false,
        false,
        Section::System,
        AttrKind::U8
    ),
    attr!(
        ConfigTelemetryPeriod,
        "Telemetry period",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigTelemetryDelay,
        "Telemetry delay",
        2,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigTelemetryDelayMin,
        "Minimum telemetry delay",
        2,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigTelemetryDelayMax,
        "Maximum telemetry delay",
        2,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigFlags,
        "Configuration flags",
        1,
        true,
        true,
        true,
        Section::Config,
        AttrKind::ConfigFlags
    ),
    attr!(
        ConfigTimezone,
        "Device timezone",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Timezone
    ),
    attr!(
        ConfigLocation,
        "Device location",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Location
    ),
    attr!(
        ConfigCls0GpioPulseDurationOc1,
        "Class 0 OC1 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls0GpioPulseDurationOc2,
        "Class 0 OC2 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls0GpioPulseDurationRl1,
        "Class 0 relay 1 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls0GpioPulseDurationRl2,
        "Class 0 relay 2 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls0GpioOutputsDefault,
        "Class 0 default output states",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Hex32
    ),
    attr!(
        ConfigCls0GpioTelemetryOnChange,
        "Class 0 telemetry-on-change mask",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Hex32
    ),
    attr!(
        ConfigCls1GpioPulseDurationPc0,
        "Class 1 PC0 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPc1,
        "Class 1 PC1 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPc2,
        "Class 1 PC2 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPc3,
        "Class 1 PC3 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPd0,
        "Class 1 PD0 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPd1,
        "Class 1 PD1 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPd2,
        "Class 1 PD2 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPd3,
        "Class 1 PD3 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei0,
        "Class 1 PEI0 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei1,
        "Class 1 PEI1 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei2,
        "Class 1 PEI2 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei3,
        "Class 1 PEI3 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei4,
        "Class 1 PEI4 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei5,
        "Class 1 PEI5 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei6,
        "Class 1 PEI6 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPei7,
        "Class 1 PEI7 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPb0,
        "Class 1 PB0 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPe0,
        "Class 1 PE0 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationPe1,
        "Class 1 PE1 pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioPulseDurationReserved,
        "Class 1 reserved pulse duration",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Milliseconds
    ),
    attr!(
        ConfigCls1GpioDirections,
        "Class 1 GPIO directions",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Hex32
    ),
    attr!(
        ConfigCls1GpioOutputsDefault,
        "Class 1 default output states",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Hex32
    ),
    attr!(
        ConfigCls1GpioTelemetryOnChange,
        "Class 1 telemetry-on-change mask",
        4,
        true,
        true,
        true,
        Section::Config,
        AttrKind::Hex32
    ),
    attr!(
        DiagResetCount,
        "Reset count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagLastResetReason,
        "Last reset reason",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::ResetReason
    ),
    attr!(
        DiagResetCountUnknown,
        "Unknown reset count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagResetCountPowerOn,
        "Power-on reset count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagResetCountWatchdog,
        "Watchdog reset count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagResetCountExternal,
        "External reset count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagLastRuntimeUptime,
        "Last runtime uptime",
        4,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::Seconds
    ),
    attr!(
        DiagLastRuntimeUptimeTotal,
        "Total runtime uptime",
        4,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::Seconds
    ),
    attr!(
        DiagLastResetStreakCount,
        "Reset streak count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagResetCountBrownOut,
        "Brown-out reset count",
        2,
        true,
        false,
        false,
        Section::Diag,
        AttrKind::U16
    ),
    attr!(
        DiagBootSignal,
        "Boot signal",
        4,
        true,
        true,
        false,
        Section::Diag,
        AttrKind::PartialCommitHash
    ),
];

pub fn get_meta_by_key(key: u16) -> Option<AttrMeta> {
    let key = AttrKey::new(key);

    ATTRS.iter().copied().find(|attr| attr.contains_key(key))
}

pub fn get_meta_by_display_name(display_name: &str) -> Option<AttrMeta> {
    ATTRS
        .iter()
        .copied()
        .find(|attr| attr.display_name.eq_ignore_ascii_case(display_name))
}

pub fn parse_attr(key: u16, raw: u32) -> Option<Attribute> {
    get_meta_by_key(key)?.parse(key, raw)
}

pub fn iter_attrs() -> impl Iterator<Item = &'static AttrMeta> {
    ATTRS.iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_decoding() {
        let key = AttrKey::new(0x3047);

        assert_eq!(key.section, Section::Diag);
        assert_eq!(key.attr, 0x04);
        assert_eq!(key.part, 0x07);
        assert_eq!(key.root(), 0x3040);
    }

    #[test]
    fn attr_id_parse() {
        let attr = AttrId::ConfigFlags.parse(0b1_10_1).unwrap();

        assert_eq!(attr.display_name(), "Configuration flags");

        match attr.value {
            AttrValue::ConfigFlags {
                error_response,
                telemetry_delay_random,
                telemetry_endpoint,
                telemetry_periodic_enabled,
            } => {
                assert!(error_response);
                assert!(!telemetry_delay_random);
                assert_eq!(telemetry_endpoint, 3);
                assert!(!telemetry_periodic_enabled);
            }
            _ => panic!("invalid attr value"),
        }
    }

    #[test]
    fn multipart_contains_valid_parts_only() {
        let meta = AttrId::IdName.meta().unwrap();

        assert_eq!(meta.parts(), 8);
        assert!(meta.contains_key(AttrKey::new(0x0020)));
        assert!(meta.contains_key(AttrKey::new(0x0027)));
        assert!(!meta.contains_key(AttrKey::new(0x0028)));
    }

    #[test]
    fn single_part_does_not_accept_part_one() {
        let meta = AttrId::ConfigFlags.meta().unwrap();

        assert_eq!(meta.parts(), 1);
        assert!(meta.contains_key(AttrKey::new(0x2040)));
        assert!(!meta.contains_key(AttrKey::new(0x2041)));
    }

    #[test]
    fn lookup_by_part_key() {
        let meta = get_meta_by_key(0x0053).unwrap();

        assert_eq!(meta.id, AttrId::IdBuildCommit);
        assert_eq!(meta.display_name, "Firmware build commit");
    }

    #[test]
    fn lookup_by_display_name() {
        let meta = get_meta_by_display_name("battery level").unwrap();

        assert_eq!(meta.id, AttrId::SystemBattery);
    }
}
