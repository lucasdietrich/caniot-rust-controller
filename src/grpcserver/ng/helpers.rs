use crate::caniot as ct;

use super::model as m;

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
