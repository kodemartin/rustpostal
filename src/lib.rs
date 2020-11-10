//! # `rustpostal`
//!
//! `libpostal` bindings for the Rust programming language.
//!
//! [libpostal]: https://github.com/openvenues/libpostal

use std::process;

pub mod address;
pub mod expand;
mod ffi;

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

pub unsafe fn setup(component: Option<&str>) {
    if !ffi::libpostal_setup() {
        process::exit(1);
    }
    match component {
        Some("expand") => {
            setup_classifier();
        }
        Some("address") => {
            setup_parser();
        }
        Some("all") => {
            setup_parser();
            setup_classifier();
        }
        Some(_) => {}
        None => {}
    }
}

pub unsafe fn teardown(component: Option<&str>) {
    ffi::libpostal_teardown();
    match component {
        Some("expand") => {
            teardown_classifier();
        }
        Some("address") => {
            teardown_parser();
        }
        Some("all") => {
            teardown_parser();
            teardown_classifier();
        }
        Some(_) => {}
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_teardown_expand() {
        unsafe {
            setup(Some("expand"));
            teardown(Some("expand"));
        }
    }

    #[test]
    fn setup_teardown_parser() {
        unsafe {
            setup(Some("address"));
            teardown(Some("address"));
        }
    }

    #[test]
    fn setup_teardown_all() {
        unsafe {
            setup(Some("all"));
            teardown(Some("all"));
        }
    }

    #[test]
    fn setup_teardown_none() {
        unsafe {
            setup(None);
            teardown(None);
        }
    }
}
