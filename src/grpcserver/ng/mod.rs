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
    tonic::include_proto!("ng");
}

mod model_heaters {
    tonic::include_proto!("ng.heaters");
}

mod model_garage {
    tonic::include_proto!("ng.garage");
}

mod model_alarms {
    tonic::include_proto!("ng.alarms");
}
