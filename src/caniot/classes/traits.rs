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

pub trait ClassTelemetryTrait: AsPayload<Ty> {
    fn to_response(self) -> caniot::ResponseData {
        caniot::ResponseData::Telemetry {
            endpoint: caniot::Endpoint::BoardControl,
            payload: self.into(),
        }
    }

    // Gets the board temperature in Celsius degrees (°C)
    //
    // # Returns
    // - `Some(f32)`: The board temperature in Celsius degrees (°C)
    fn get_board_temperature(&self) -> Option<f32>;

    fn get_outside_temperature(&self) -> Option<f32>;
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
}
