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
use std::collections::HashMap;
use std::ffi::{CStr, CString, NulError};
use std::slice::Iter;
use std::vec::IntoIter;

use crate::ffi;

/// Represents the parsing result.
#[derive(Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
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

impl IntoIterator for AddressParserResponse {
    type Item = (String, String);
    type IntoIter = std::iter::Zip<IntoIter<String>, IntoIter<String>>;

    /// Iterates over `(label, token)` pairs by consuming the value.
    fn into_iter(self) -> Self::IntoIter {
        self.labels.into_iter().zip(self.tokens.into_iter())
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
#[derive(Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct AddressParserOptions {
    language: Option<CString>,
    country: Option<CString>,
}

impl AddressParserOptions {
    /// Create options for the address parser.
    ///
    /// # Examples
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

/// A parsed address backed by a `HashMap`.
/// The only way to make one is from an `AddressParserResponse`.
/// It implements a getter method for each label that might
/// be included in the `AddressParserResponse`.
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct ParsedAddress {
    label_to_token: HashMap<String, String>,
}

impl ParsedAddress {
    pub fn house(&self) -> Option<String> {
        self.label_to_token.get("house").cloned()
    }

    pub fn house_number(&self) -> Option<String> {
        self.label_to_token.get("house_number").cloned()
    }

    pub fn po_box(&self) -> Option<String> {
        self.label_to_token.get("po_box").cloned()
    }

    pub fn building(&self) -> Option<String> {
        self.label_to_token.get("building").cloned()
    }

    pub fn entrance(&self) -> Option<String> {
        self.label_to_token.get("entrance").cloned()
    }

    pub fn staircase(&self) -> Option<String> {
        self.label_to_token.get("staircase").cloned()
    }

    pub fn level(&self) -> Option<String> {
        self.label_to_token.get("level").cloned()
    }

    pub fn unit(&self) -> Option<String> {
        self.label_to_token.get("unit").cloned()
    }

    pub fn road(&self) -> Option<String> {
        self.label_to_token.get("road").cloned()
    }

    pub fn metro_station(&self) -> Option<String> {
        self.label_to_token.get("metro_station").cloned()
    }

    pub fn suburb(&self) -> Option<String> {
        self.label_to_token.get("suburb").cloned()
    }

    pub fn city_district(&self) -> Option<String> {
        self.label_to_token.get("city_district").cloned()
    }

    pub fn city(&self) -> Option<String> {
        self.label_to_token.get("city").cloned()
    }

    pub fn state_district(&self) -> Option<String> {
        self.label_to_token.get("state_district").cloned()
    }

    pub fn island(&self) -> Option<String> {
        self.label_to_token.get("island").cloned()
    }

    pub fn state(&self) -> Option<String> {
        self.label_to_token.get("state").cloned()
    }

    // postcode may be referred to as postal_code somewheres
    // https://github.com/openvenues/libpostal/blob/9c975972985b54491e756efd70e416f18ff97958/src/address_parser.h#L122
    pub fn postcode(&self) -> Option<String> {
        self.label_to_token.get("postcode").cloned()
    }

    pub fn country_region(&self) -> Option<String> {
        self.label_to_token.get("country_region").cloned()
    }

    pub fn country(&self) -> Option<String> {
        self.label_to_token.get("country").cloned()
    }

    pub fn world_region(&self) -> Option<String> {
        self.label_to_token.get("world_region").cloned()
    }

    pub fn website(&self) -> Option<String> {
        self.label_to_token.get("website").cloned()
    }

    pub fn telephone(&self) -> Option<String> {
        self.label_to_token.get("telephone").cloned()
    }
}

impl From<AddressParserResponse> for ParsedAddress {
    /// Create a new `ParsedAddress` from an `AddressParserResponse`.
    fn from(response: AddressParserResponse) -> Self {
        let mut parsed_address = ParsedAddress::default();
        for (label, token) in response {
            parsed_address.label_to_token.insert(label, token);
        }
        parsed_address
    }
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

    #[test]
    fn test_parsed_address_default() {
        let parsed_address = ParsedAddress::default();
        assert_eq!(parsed_address.house(), None);
        assert_eq!(parsed_address.house_number(), None);
        assert_eq!(parsed_address.po_box(), None);
        assert_eq!(parsed_address.building(), None);
        assert_eq!(parsed_address.entrance(), None);
        assert_eq!(parsed_address.staircase(), None);
        assert_eq!(parsed_address.level(), None);
        assert_eq!(parsed_address.unit(), None);
        assert_eq!(parsed_address.road(), None);
        assert_eq!(parsed_address.metro_station(), None);
        assert_eq!(parsed_address.suburb(), None);
        assert_eq!(parsed_address.city_district(), None);
        assert_eq!(parsed_address.city(), None);
        assert_eq!(parsed_address.state_district(), None);
        assert_eq!(parsed_address.island(), None);
        assert_eq!(parsed_address.state(), None);
        assert_eq!(parsed_address.postcode(), None);
        assert_eq!(parsed_address.country_region(), None);
        assert_eq!(parsed_address.country(), None);
        assert_eq!(parsed_address.world_region(), None);
        assert_eq!(parsed_address.website(), None);
        assert_eq!(parsed_address.telephone(), None);
    }
}
