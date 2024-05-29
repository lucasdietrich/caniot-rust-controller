pub mod iface;

pub use iface::*;

#[cfg(feature = "emu")]
pub mod emu;

#[cfg(feature = "emu")]
pub use emu::*;

#[cfg(not(feature = "emu"))]
pub mod can;

#[cfg(feature = "emu")]
pub type IFaceType = crate::bus::emu::CanInterface;
#[cfg(not(feature = "emu"))]
pub type IFaceType = crate::bus::can::CanInterface;
