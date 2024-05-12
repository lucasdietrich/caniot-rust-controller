pub trait Class<'a> {
    type Telemetry: ClassTelemetryTrait<'a>;
    type Command: ClassCommandTrait<'a>;
}

pub trait ClassTelemetryTrait<'a>: TryFrom<&'a [u8]> + Into<Vec<u8>> {}

pub trait ClassCommandTrait<'a>: TryFrom<&'a [u8]> + Into<Vec<u8>> {}
