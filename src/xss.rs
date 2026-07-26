// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use super::xlib::{
    Bool, Display, Time, Window,
};
use std::os::raw::{c_int, c_ulong};

//
// types
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XScreenSaverInfo {
    pub window: Window,
    pub state: c_int,
    pub kind: c_int,
    pub til_or_since: c_ulong,
    pub idle: c_ulong,
    pub eventMask: c_ulong,
}

//
// event structures
//

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XScreenSaverNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub state: c_int,
    pub kind: c_int,
    pub forced: Bool,
    pub time: Time,
}

pub const ScreenSaverName: &str = "MIT-SCREEN-SAVER";
pub const ScreenSaverPropertyName: &str = "_MIT_SCREEN_SAVER_ID";

pub const ScreenSaverNotifyMask: c_ulong = 0x00000001;
pub const ScreenSaverCycleMask: c_ulong = 0x00000002;

pub const ScreenSaverMajorVersion: c_int = 1;
pub const ScreenSaverMinorVersion: c_int = 1;

pub const ScreenSaverOff: c_int = 0;
pub const ScreenSaverOn: c_int = 1;
pub const ScreenSaverCycle: c_int = 2;
pub const ScreenSaverDisabled: c_int = 3;

pub const ScreenSaverBlanked: c_int = 0;
pub const ScreenSaverInternal: c_int = 1;
pub const ScreenSaverExternal: c_int = 2;

pub const ScreenSaverNotify: c_int = 0;
pub const ScreenSaverNumberEvents: c_int = 1;
