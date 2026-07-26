// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(deref_nullptr)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
mod link;
mod internal;

pub mod xlib;

// necessary for compiling
pub mod xss;
pub mod xf86vmode;
pub mod xrender;
pub mod xrandr {
    include!("xrandr.rs");
    include!("old_xrandr.rs");
}

// slowdown ratio stays the same
// pub mod keysym;
// pub mod xcursor;
// pub mod xft;

// some of them necessary for slowdown
// pub mod dpms;
// pub mod glx;
// pub mod sync;
// pub mod xfixes;
// pub mod xinerama;
// pub mod xinput;
// pub mod xinput2;
// pub mod xlib_xcb;
// pub mod xmd;
// pub mod xmu;
// pub mod xpresent;
// pub mod xrecord;
// pub mod xshm;
// biggest slowdown by far
pub mod xt;

