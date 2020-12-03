//! # `rustpostal`
//!
//! [`libpostal`][libpostal] bindings for the Rust programming language.
//!
//! [libpostal]: https://github.com/openvenues/libpostal

use std::process;

use self::LibModules::*;

pub mod address;
pub mod expand;
mod ffi;

/// Library modules to setup and teardown, at the start
/// and at the end of our program.
pub enum LibModules {
    Address,
    Expand,
    All,
}

unsafe fn setup_parser() {
    if !ffi::libpostal_setup_parser() {
        process::exit(1)
    };
}

unsafe fn setup_classifier() {
    if !ffi::libpostal_setup_language_classifier() {
        process::exit(1)
    };
}

unsafe fn teardown_parser() {
    ffi::libpostal_teardown_parser();
}

unsafe fn teardown_classifier() {
    ffi::libpostal_teardown_language_classifier();
}

/// Setup the necessary `libpostal` components.
///
/// # Safety
///
/// The method should be complemented by [`teardown`](self::teardown)
/// to make the calling program safe.
pub unsafe fn setup(component: LibModules) {
    if !ffi::libpostal_setup() {
        process::exit(1);
    }
    match component {
        Expand => {
            setup_classifier();
        }
        Address => {
            setup_parser();
        }
        All => {
            setup_parser();
            setup_classifier();
        }
    }
}

/// Teardown initialized `libpostal` components.
///
/// # Safety
///
/// The method should follow [`setup`](self::setup) and makes the calling
/// program safe.
pub unsafe fn teardown(component: LibModules) {
    ffi::libpostal_teardown();
    match component {
        Expand => {
            teardown_classifier();
        }
        Address => {
            teardown_parser();
        }
        All => {
            teardown_parser();
            teardown_classifier();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_teardown_expand() {
        unsafe {
            setup(Expand);
            teardown(Expand);
        }
    }

    #[test]
    fn setup_teardown_parser() {
        unsafe {
            setup(Address);
            teardown(Address);
        }
    }

    #[test]
    fn setup_teardown_all() {
        unsafe {
            setup(All);
            teardown(All);
        }
    }
}
