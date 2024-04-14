use num_derive::FromPrimitive;
use serde::Serialize;

pub const ERROR_BASE: isize = 0x3A00;

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive, Serialize)]
pub enum ErrorCode {
    Ok = 0x00000000,
    Einval = ERROR_BASE, // Invalid argument
    Enproc,              // UNPROCESSABLE
    Ecmd,                // COMMAND
    Ekey,                // KEY (read/write-attribute)
    Etimeout,            // TIMEOUT
    Eagain,              // BUSY / EAGAIN
    Efmt,                // FORMAT
    Ehandlerc,           // UNDEFINED COMMAND HANDLER
    Ehandlert,           // UNDEFINED TELEMETRY HANDLER
    Etelemetry,          // TELEMETRY
    Eunexpected,         // Unexpected frame
    Eep,                 // ENDPOINT
    Ecmdep,              // ILLEGAL COMMAND, BROADCAST TO ALL ENDPOINTS
    Euninit,             // NOT INITIALIZED
    Edriver,             // DRIVER
    Eapi,                // API
    Ekeysection,         // Unknown attributes section
    Ekeyattr,            // Unknown attribute
    Ekeypart,            // Unknown attribute part
    Enoattr,             // No attribute
    Eclsattr,            // Class attribute not accessible for current device
    Ereadonly,
    Enull,
    EnullDrv,
    EnullApi,
    EnullId,
    EnullDev,
    EnullCfg,
    EnullCtrl,
    EnullCtrlCb,
    Eroattr,    // READ-ONLY ATTRIBUTE
    Ereadattr,  // QUERY READ ATTR
    Ewriteattr, // QUERY WRITE ATTR
    Eenocb,     // no event handler
    Eecb,       // ECCB
    Epqalloc,   // PENDING QUERY ALLOCATION
    Enopq,      // NO PENDING QUERY
    Enohandle,  // NO HANDLER
    Edevice,    // DEVICE
    Eframe,     // FRAME, not a valid caniot frame
    Emlfrm,     // MALFORMED FRAME
    Eclass,     // INVALID CLASS
    Ecfg,       // INVALID CONFIGURATION
    Ehyst,      // Invalid hysteresis structure
    Enotsup,    // NOT SUPPORTED
    Enimpl,     // NOT IMPLEMENTED
}

impl ErrorCode {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}
