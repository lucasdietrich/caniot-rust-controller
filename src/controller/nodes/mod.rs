pub mod demo;
pub mod garage;
pub mod heaters;
pub mod outdoor_alarm;

pub use demo::*;
pub use garage::*;
pub use heaters::*;
pub use outdoor_alarm::*;

#[cfg(test)]
mod garage_test;
