// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::c_int;

use super::xmd::{CARD16};


pub const DPMSMajorVersion: c_int = 1;
pub const DPMSMinorVersion: c_int = 1;

pub const DPMSExtensionName: &str = "DPMS";

pub const DPMSModeOn: CARD16 = 0;
pub const DPMSModeStandby: CARD16 = 1;
pub const DPMSModeSuspend: CARD16 = 2;
pub const DPMSModeOff: CARD16 = 3;
