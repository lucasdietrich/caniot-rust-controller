pub mod alarms_service;
#[cfg(feature = "ble-copro")]
pub mod copro_service;
pub mod devices_service;
pub mod garage_service;
pub mod heaters_service;
pub mod helpers;
pub mod internal_service;

pub use alarms_service::*;
#[cfg(feature = "ble-copro")]
pub use copro_service::*;
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

    #[cfg(feature = "ble-copro")]
    pub mod copro {
        tonic::include_proto!("ng.copro");
    }
}
