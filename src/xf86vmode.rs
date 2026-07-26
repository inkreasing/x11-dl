// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_float, c_int, c_uchar, c_uint, c_ulong, c_ushort};

use super::xlib::{Bool, Display, Time, Window};


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeGamma {
    pub red: c_float,
    pub green: c_float,
    pub blue: c_float,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XF86VidModeModeInfo {
    pub dotclock: c_uint,
    pub hdisplay: c_ushort,
    pub hsyncstart: c_ushort,
    pub hsyncend: c_ushort,
    pub htotal: c_ushort,
    pub hskew: c_ushort,
    pub vdisplay: c_ushort,
    pub vsyncstart: c_ushort,
    pub vsyncend: c_ushort,
    pub vtotal: c_ushort,
    pub flags: c_uint,
    pub privsize: c_int,
    pub private: *mut i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeModeLine {
    pub hdisplay: c_ushort,
    pub hsyncstart: c_ushort,
    pub hsyncend: c_ushort,
    pub htotal: c_ushort,
    pub hskew: c_ushort,
    pub vdisplay: c_ushort,
    pub vsyncstart: c_ushort,
    pub vsyncend: c_ushort,
    pub vtotal: c_ushort,
    pub flags: c_uint,
    pub privsize: c_int,
    pub private: *mut i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeMonitor {
    pub vendor: *mut c_char,
    pub model: *mut c_char,
    pub EMPTY: c_float,
    pub nhsync: c_uchar,
    pub hsync: *mut XF86VidModeSyncRange,
    pub nvsync: c_uchar,
    pub vsync: *mut XF86VidModeSyncRange,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeSyncRange {
    pub hi: c_float,
    pub lo: c_float,
}

//
// event structures
//

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XF86VidModeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub root: Window,
    pub state: c_int,
    pub kind: c_int,
    pub forced: Bool,
    pub time: Time,
}

