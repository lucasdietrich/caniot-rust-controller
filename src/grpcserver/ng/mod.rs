pub mod alarms_service;
pub mod controller_service;
pub mod devices_service;
pub mod garage_service;
pub mod heaters_service;
pub mod helpers;
pub mod internal_service;

pub use alarms_service::*;
pub use controller_service::*;
pub use devices_service::*;
pub use garage_service::*;
pub use heaters_service::*;
pub use internal_service::*;

mod model {
    // Only a single proto file can be included in a module
    tonic::include_proto!("ng"); // commons

    // proto files must be hierarchically structured when referencing each other

    pub mod internal {
        tonic::include_proto!("ng.internal");
    }

    pub mod common {
        tonic::include_proto!("ng.common");
    }

    pub mod controller {
        tonic::include_proto!("ng.controller");
    }

    pub mod heaters {
        tonic::include_proto!("ng.heaters");
    }

    pub mod garage {
        tonic::include_proto!("ng.garage");
    }

    pub mod alarms {
        tonic::include_proto!("ng.alarms");
    }

    pub mod devices {
        tonic::include_proto!("ng.devices");
    }
}

mod model_alarms {
    tonic::include_proto!("ng.alarms");
}
