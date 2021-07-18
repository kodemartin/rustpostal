//! # `rustpostal`
//!
//! [`libpostal`][libpostal] bindings for the Rust programming language.
//!
//! [libpostal]: https://github.com/openvenues/libpostal

use std::process;

use self::LibModules::*;

pub mod address;
pub mod error;
pub mod expand;
mod ffi;

use error::SetupError;

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

impl LibModules {
    /// Setup the necessary `libpostal` resources.
    ///
    /// # Examples
    /// ```
    /// use rustpostal::error::SetupError;
    /// use rustpostal::LibModules;
    ///
    /// fn main() -> Result<(), SetupError> {
    ///     let postal_module = LibModules::Expand;
    ///     postal_module.setup()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn setup(&self) -> Result<(), SetupError> {
        if unsafe { !ffi::libpostal_setup() } {
            return Err(SetupError);
        }
        match self {
            Expand => unsafe {
                setup_classifier();
            },
            Address => unsafe {
                setup_parser();
            },
            All => unsafe {
                setup_parser();
                setup_classifier();
            },
        }
        Ok(())
    }
}

impl Drop for LibModules {
    /// Tear down the ffi resources that were initialized during setup.
    fn drop(&mut self) {
        unsafe { ffi::libpostal_teardown() };
        match self {
            Expand => unsafe { teardown_classifier() },
            Address => unsafe { teardown_parser() },
            All => unsafe {
                teardown_parser();
                teardown_classifier();
            },
        }
    }
}

/// Setup the necessary `libpostal` components.
///
/// # Safety
///
/// The method should be complemented by [`teardown`](self::teardown)
/// to make the calling program safe.
#[deprecated(
    since = "0.2.0",
    note = "Please use the `setup` method in `LibModules` instead"
)]
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
#[deprecated(
    since = "0.2.0",
    note = "This can be handled by the `Drop` traint when `LibModules` values go out of scope"
)]
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
    fn libmodules_setup_expand() {
        let postal_module = Expand;
        assert!(postal_module.setup().is_ok());
    }

    #[test]
    fn libmodules_setup_parser() {
        let postal_module = Address;
        assert!(postal_module.setup().is_ok());
    }

    #[test]
    fn libmodules_setup_all() {
        let postal_module = All;
        assert!(postal_module.setup().is_ok());
    }
}
