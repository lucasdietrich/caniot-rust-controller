pub mod class0;
pub mod class1;

pub mod command;
pub mod telemetry;
pub mod traits;
pub mod utils;

pub use command::BoardClassCommand;
pub use telemetry::BoardClassTelemetry;

#[cfg(test)]
mod class0_test;

#[cfg(test)]
mod class1_test;
