use crate::caniot as ct;

use super::model as ng;

impl Into<ng::DeviceId> for ct::DeviceId {
    fn into(self) -> ng::DeviceId {
        ng::DeviceId {
            did: self.to_u8() as u32,
        }
    }
}

impl Into<ct::DeviceId> for ng::DeviceId {
    fn into(self) -> ct::DeviceId {
        ct::DeviceId::try_from_u8(self.did as u8).unwrap()
    }
}

impl From<ng::Endpoint> for ct::Endpoint {
    fn from(value: ng::Endpoint) -> Self {
        match value {
            ng::Endpoint::AppDefault => ct::Endpoint::ApplicationDefault,
            ng::Endpoint::App1 => ct::Endpoint::Application1,
            ng::Endpoint::App2 => ct::Endpoint::Application2,
            ng::Endpoint::BoardLevelControl => ct::Endpoint::BoardControl,
        }
    }
}

impl From<ng::TwoStatePulse> for ct::TSP {
    fn from(value: ng::TwoStatePulse) -> Self {
        match value {
            ng::TwoStatePulse::TspNone => ct::TSP::None,
            ng::TwoStatePulse::TspSet => ct::TSP::Set,
            ng::TwoStatePulse::TspReset => ct::TSP::Reset,
            ng::TwoStatePulse::TspPulse => ct::TSP::Pulse,
        }
    }
}

impl Into<ng::CaniotFrame> for ct::Response {
    fn into(self) -> ng::CaniotFrame {
        ng::CaniotFrame {
            did: Some(self.device_id.into()),
            payload: self.get_can_payload().iter().map(|b| *b as u32).collect(),
        }
    }
}

impl Into<ng::DeviceIdInfos> for ct::DeviceId {
    fn into(self) -> ng::DeviceIdInfos {
        ng::DeviceIdInfos {
            obj: Some(ng::DeviceId {
                did: self.to_u8() as u32,
            }),
            did: self.to_u8() as u32,
            sid: self.sub_id as u32,
            cls: self.class as u32,
        }
    }
}
