use crate::caniot::{self, AsPayload, Cd, ClCd, Payload, Ty};

pub trait Class {
    const CLASS_ID: u8;

    type Telemetry: ClassTelemetryTrait;
    type Command: ClassCommandTrait;

    // layout functions
    // inputs count
    // outputs count
    // temperatures count
    //
}

#[derive(Debug, Clone, Copy)]
pub enum TempSensType {
    BoardSensor,
    ExternalSensor(u8),
    AvgExternal,
    AnyExternal,
    Any,
}

pub trait ClassTelemetryTrait: AsPayload<Ty> {
    fn to_response(self) -> caniot::ResponseData {
        caniot::ResponseData::Telemetry {
            endpoint: caniot::Endpoint::BoardControl,
            payload: self.into(),
        }
    }

    fn get_temperature(&self, sensor: TempSensType) -> Option<f32>;
}

impl From<Payload<ClCd>> for Payload<Cd> {
    fn from(value: Payload<ClCd>) -> Self {
        Self::new_unchecked(value.data())
    }
}

pub trait ClassCommandTrait: AsPayload<ClCd> + Default + Clone {
    fn into_request(&self) -> caniot::RequestData {
        caniot::RequestData::Command {
            endpoint: caniot::Endpoint::BoardControl,
            payload: self.to_payload().into(),
        }
    }

    // Returns whether the command actually does something, i.e. sending the command has an effect on the device.
    fn has_effect(&self) -> bool;
}
