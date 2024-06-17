use crate::caniot::{self, AsPayload, ClassCommandPL, Payload, TelemetryPL};

pub trait Class {
    type Telemetry: ClassTelemetryTrait;
    type Command: ClassCommandTrait;

    fn get_class_id() -> u8;

    // layout functions
    // inputs count
    // outputs count
    // temperatures count
    //
}

pub trait ClassTelemetryTrait: AsPayload<TelemetryPL> {
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
}

pub trait ClassCommandTrait: AsPayload<ClassCommandPL> {
    fn to_request(self) -> caniot::RequestData {
        caniot::RequestData::Command {
            endpoint: caniot::Endpoint::BoardControl,
            payload: Into::<Payload<ClassCommandPL>>::into(self).into(),
        }
    }
}
