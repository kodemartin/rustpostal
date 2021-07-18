//! Parsing utilities for postal addresses.
//!
//! # Examples
//!
//! ```
//! use rustpostal::{address, LibModules};
//! use rustpostal::error::RuntimeError;
//!
//! fn main() -> Result<(), RuntimeError> {
//!     let postal_module = LibModules::Address;
//!     postal_module.setup()?;
//!
//!     let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
//!
//!     let labeled_tokens = address::parse_address(address, None, None)?;
//!
//!     for (label, token) in &labeled_tokens {
//!         println!("{}: {}", label, token);
//!     }
//!     Ok(())
//! }
//! ```
use std::ffi::{CStr, CString, NulError};
use std::slice::Iter;

use crate::ffi;

/// Represents the parsing result.
#[derive(Clone, Default, Debug, Hash)]
pub struct AddressParserResponse {
    tokens: Vec<String>,
    labels: Vec<String>,
}

impl AddressParserResponse {
    /// Create a new value.
    pub fn new() -> AddressParserResponse {
        Default::default()
    }
}

impl<'a> IntoIterator for &'a AddressParserResponse {
    type Item = (&'a String, &'a String);
    type IntoIter = std::iter::Zip<Iter<'a, String>, Iter<'a, String>>;

    /// Iterates over `(label, token)` pairs.
    fn into_iter(self) -> Self::IntoIter {
        self.labels[..].iter().zip(self.tokens[..].iter())
    }
}

/// Parsing options.
#[derive(Default)]
pub struct AddressParserOptions {
    language: Option<CString>,
    country: Option<CString>,
}

impl AddressParserOptions {
    /// Create options for the address parser.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rustpostal::address;
    ///
    /// fn main() -> Result<(), std::ffi::NulError> {
    ///     let options = address::AddressParserOptions::new(Some("en"), Some("en"))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        language: Option<&str>,
        country: Option<&str>,
    ) -> Result<AddressParserOptions, NulError> {
        let mut options = AddressParserOptions::default();
        if let Some(s) = language {
            let c_lang = CString::new(s)?;
            options.language = Some(c_lang);
        }
        if let Some(s) = country {
            let c_country = CString::new(s)?;
            options.country = Some(c_country);
        }
        Ok(options)
    }

    /// Get the language option.
    pub fn language(&self) -> Option<&str> {
        if let Some(language) = &self.language {
            return Some(language.to_str().unwrap());
        }
        None
    }

    /// Get the country option.
    pub fn country(&self) -> Option<&str> {
        if let Some(country) = &self.country {
            return Some(country.to_str().unwrap());
        }
        None
    }

    fn update_ffi_language(&self, options: &mut ffi::libpostal_address_parser_options) {
        if let Some(language) = &self.language {
            options.language = language.as_ptr();
        };
    }

    fn update_ffi_country(&self, options: &mut ffi::libpostal_address_parser_options) {
        if let Some(country) = &self.country {
            options.country = country.as_ptr();
        };
    }

    /// Parse a postal address and derive labeled tokens using `libpostal`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustpostal::error::RuntimeError;
    /// use rustpostal::{address, LibModules};
    ///
    /// fn main() -> Result<(), RuntimeError> {
    ///     let postal_module = LibModules::Address;
    ///     postal_module.setup()?;
    ///     
    ///     let options = address::AddressParserOptions::new(None, None)?;
    ///     let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
    ///     let labels_tokens = options.parse(address)?;
    ///
    ///     for (label, token) in &labels_tokens {
    ///         println!("{}: {}", label, token);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// It will return an error if the address contains an internal null byte.
    pub fn parse<'b>(&self, address: &'b str) -> Result<AddressParserResponse, NulError> {
        let c_address = CString::new(address)?;
        let mut response = AddressParserResponse::new();
        let ptr = c_address.into_raw();

        let mut ffi_options = unsafe { ffi::libpostal_get_address_parser_default_options() };
        self.update_ffi_language(&mut ffi_options);
        self.update_ffi_country(&mut ffi_options);

        let raw = unsafe { ffi::libpostal_parse_address(ptr, ffi_options) };
        if let Some(parsed) = unsafe { raw.as_ref() } {
            for i in 0..parsed.num_components {
                let component = unsafe { CStr::from_ptr(*parsed.components.add(i)) };
                let label = unsafe { CStr::from_ptr(*parsed.labels.add(i)) };
                response
                    .tokens
                    .push(String::from(component.to_str().unwrap()));
                response.labels.push(String::from(label.to_str().unwrap()));
            }
        };
        unsafe {
            ffi::libpostal_address_parser_response_destroy(raw);
        }
        let _c_address = unsafe { CString::from_raw(ptr) };
        Ok(response)
    }
}

/// Analyze address into labeled tokens.
///
/// * `address`: The postal address to parse.
/// * `language`: A language code.
/// * `country`: A country code.
///
/// The function wraps [`AddressParserOptions::parse`].
pub fn parse_address(
    address: &str,
    language: Option<&str>,
    country: Option<&str>,
) -> Result<AddressParserResponse, NulError> {
    let options = AddressParserOptions::new(language, country)?;
    options.parse(address)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::RuntimeError;
    use crate::LibModules;

    #[test]
    fn default_address_parser_options() -> Result<(), NulError> {
        let options = AddressParserOptions::new(None, None)?;
        assert_eq!(options.language(), None);
        assert_eq!(options.country(), None);
        let options = AddressParserOptions::new(None, Some("EN"))?;
        assert_eq!(options.language(), None);
        assert_eq!(options.country(), Some("EN"));
        Ok(())
    }

    #[test]
    fn address_parser_options_parse() -> Result<(), RuntimeError> {
        let postal_module = LibModules::Address;
        postal_module.setup()?;

        let options = AddressParserOptions::new(None, None)?;
        let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";

        let labeled_tokens = options.parse(address)?;

        for (label, token) in &labeled_tokens {
            println!("{}: {}", label, token);
        }
        Ok(())
    }
}
