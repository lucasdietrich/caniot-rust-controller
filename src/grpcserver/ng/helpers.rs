use crate::caniot as ct;

use super::model as m;

impl Into<m::DeviceId> for ct::DeviceId {
    fn into(self) -> m::DeviceId {
        m::DeviceId {
            did: self.to_u8() as u32,
        }
    }
}

impl From<m::Endpoint> for ct::Endpoint {
    fn from(value: m::Endpoint) -> Self {
        match value {
            m::Endpoint::AppDefault => ct::Endpoint::ApplicationDefault,
            m::Endpoint::App1 => ct::Endpoint::Application1,
            m::Endpoint::App2 => ct::Endpoint::Application2,
            m::Endpoint::BoardLevelControl => ct::Endpoint::BoardControl,
        }
    }
}

impl From<m::TwoStatePulse> for ct::TSP {
    fn from(value: m::TwoStatePulse) -> Self {
        match value {
            m::TwoStatePulse::TspNone => ct::TSP::None,
            m::TwoStatePulse::TspSet => ct::TSP::Set,
            m::TwoStatePulse::TspReset => ct::TSP::Reset,
            m::TwoStatePulse::TspPulse => ct::TSP::Pulse,
        }
    }
}

impl Into<m::CaniotFrame> for ct::Response {
    fn into(self) -> m::CaniotFrame {
        m::CaniotFrame {
            did: Some(self.device_id.into()),
            payload: self.get_can_payload().iter().map(|b| *b as u32).collect(),
        }
    }
}
