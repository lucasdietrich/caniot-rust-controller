mod actions;
pub mod outdoor;
mod types;

pub use actions::{Action, AlarmEnable, LightAction, LightsActions, SirenAction};
pub use outdoor::*;

#[cfg(test)]
mod outdoor_test;
