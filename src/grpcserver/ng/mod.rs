pub mod devices_service;
pub mod internal_service;

pub use devices_service::*;
pub use internal_service::*;

mod model {
    tonic::include_proto!("ng");
}
