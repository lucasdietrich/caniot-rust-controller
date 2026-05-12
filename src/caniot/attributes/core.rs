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
}

impl From<AttrId> for u16 {
    fn from(id: AttrId) -> Self {
        id.key()
    }
}

impl From<AttrId> for AttrKey {
    fn from(id: AttrId) -> Self {
        AttrKey::new(id.key())
    }
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
}

impl AttrMeta {
    pub const fn key(self) -> u16 {
        self.id.key()
    }

    pub const fn parts(self) -> u8 {
        (self.size + 3) / 4
    }

    pub fn contains_key(self, key: AttrKey) -> bool {
        let my = AttrKey::new(self.id.key());
        my.section as u8 == key.section as u8 && my.attr == key.attr && key.part < self.parts()
    }
}

pub trait Attribute {
    const META: AttrMeta;
    #[allow(non_upper_case_globals)]
    const attributeMeta: AttrMeta = Self::META;
    type Value: fmt::Display + Clone + PartialEq + Eq;

    fn parse_value(key: AttrKey, raw: u32) -> Self::Value;
}
