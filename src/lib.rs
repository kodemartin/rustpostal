//! # `rustpostal`
//!
//! `libpostal` bindings for the Rust programming language.
//!
//! [libpostal]: https://github.com/openvenues/libpostal

use std::process;

pub mod address;
mod ffi;

pub unsafe fn setup() {
    if !ffi::libpostal_setup() || !ffi::libpostal_setup_parser() {
        process::exit(1);
    }
}

pub unsafe fn teardown() {
    ffi::libpostal_teardown();
    ffi::libpostal_teardown_parser();
}
