//! Runtime errors for the [`rustpostal`] crate.

use std::error;
use std::ffi::NulError;
use std::fmt;

/// An error indicating failure in setting up required `libpostal` resources.
/// Returned by [`setup`](`LibModules.setup`) method on [`LibModules`].
#[derive(Debug, Clone)]
pub struct SetupError;

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "setup of libpostal resources failed")
    }
}

impl error::Error for SetupError {}

/// Error indicating possible runtime failures.
#[derive(Debug, Clone)]
pub enum RuntimeError {
    FailedSetup(SetupError),
    InvalidAddress(NulError),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::FailedSetup(ref err) => err.fmt(f),
            RuntimeError::InvalidAddress(_) => {
                write!(f, "input address possibly contains internal null byte")
            }
        }
    }
}

impl error::Error for RuntimeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RuntimeError::FailedSetup(ref err) => Some(err),
            RuntimeError::InvalidAddress(ref err) => Some(err),
        }
    }
}

impl From<SetupError> for RuntimeError {
    /// Create a new [`RuntimeError`] consuming a [`SetupError`].
    fn from(err: SetupError) -> Self {
        RuntimeError::FailedSetup(err)
    }
}

impl From<NulError> for RuntimeError {
    /// Create a new [`RuntimeError`] consuming a [`NulError`].
    fn from(err: NulError) -> Self {
        RuntimeError::InvalidAddress(err)
    }
}
