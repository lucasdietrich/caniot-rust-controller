use crate::caniot;

pub trait Class<'a> {
    type Telemetry: ClassTelemetryTrait<'a>;
    type Command: ClassCommandTrait<'a>;
}

pub trait ClassTelemetryTrait<'a>: TryFrom<&'a [u8]> + Into<Vec<u8>> {
    fn to_response(self) -> caniot::ResponseData {
        caniot::ResponseData::Telemetry {
            endpoint: caniot::Endpoint::BoardControl,
            payload: self.into(),
        }
    }
}

pub trait ClassCommandTrait<'a>: TryFrom<&'a [u8]> + Into<Vec<u8>> {
    fn to_request(self) -> caniot::RequestData {
        caniot::RequestData::Command {
            endpoint: caniot::Endpoint::BoardControl,
            payload: self.into(),
        }
    }
}
