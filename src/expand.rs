//! Normalization utilities.
//!
//! # Examples
//!
//! ```
//! use rustpostal::LibModules;
//! use rustpostal::expand;
//!
//! fn main() {
//!     unsafe { rustpostal::setup(LibModules::Expand) }
//!
//!     let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
//!
//!     let expanded = expand::expand_address_with_options(address, Some(vec!["en"]));
//!
//!     for expansion in expanded {
//!         println!("{}", expansion);
//!     }
//!
//!     unsafe { rustpostal::teardown(LibModules::Expand) }
//! }
//! ```
#![allow(unused)]

use bitflags::bitflags;
use std::ffi::{CStr, CString};

use crate::ffi;

bitflags! {
    #[derive(Default)]
    pub struct StringOptions: u32 {
        const TRANSLITERATE = 1 << 0;
        const STRIP_ACCENTS = 1 << 1;
        const DECOMPOSE = 1 << 2;
        const LOWERCASE = 1 << 3;
        const TRIM_STRING = 1 << 4;
        const DROP_PARENTHETICALS = 1 << 5;
        const REPLACE_NUMERIC_HYPHENS = 1 << 6;
        const DELETE_NUMERIC_HYPHENS = 1 << 7;
        const SPLIT_ALPHA_FROM_NUMERIC = 1 << 8;
        const REPLACE_WORD_HYPHENS = 1 << 9;
        const DELETE_WORD_HYPHENS = 1 << 10;
        const DELETE_FINAL_PERIODS = 1 << 11;
        const DELETE_ACRONYM_PERIODS = 1 << 12;
        const DROP_ENGLISH_POSSESSIVES = 1 << 13;
        const DELETE_APOSTROPHES = 1 << 14;
        const EXPAND_NUMEX = 1 << 15;
        const ROMAN_NUMERALS = 1 << 16;
        const LATIN_ASCII = 1 << 17;
    }
}

bitflags! {
    /// Bit set of active address components in normalization options.
    pub struct AddressComponents: u16 {
        const NONE = 0;
        const ANY = 1 << 0;
        const NAME = 1 << 1;
        const HOUSE_NUMBER = 1 << 2;
        const STREET = 1 << 3;
        const UNIT = 1 << 4;
        const LEVEL = 1 << 5;
        const STAIRCASE = 1 << 6;
        const ENTRANCE = 1 << 7;
        const CATEGORY = 1 << 8;
        const NEAR = 1 << 9;
        const TOPONYM = 1 << 13;
        const POSTAL_CODE = 1 << 14;
        const PO_BOX = 1 << 15;
        }
}

impl Default for AddressComponents {
    /// Union of `NAME, HOUSE_NUMBER, STREET, PO_BOX, UNIT,
    /// LEVEL, ENTRANCE, STAIRCASE, POSTAL_CODE`.
    fn default() -> Self {
        Self::NAME
            | Self::HOUSE_NUMBER
            | Self::STREET
            | Self::PO_BOX
            | Self::UNIT
            | Self::LEVEL
            | Self::ENTRANCE
            | Self::STAIRCASE
            | Self::POSTAL_CODE
    }
}

/// Normalization options.
#[derive(Clone, Debug)]
pub struct NormalizeOptions<'a> {
    pub languages: Option<Vec<&'a str>>,
    language_c_strs: Option<Vec<CString>>,
    language_ptrs: Option<Vec<*const libc::c_char>>,
    pub address_components: AddressComponents,
    // String options
    pub string_options: StringOptions,
}

impl<'a> Default for NormalizeOptions<'a> {
    fn default() -> NormalizeOptions<'a> {
        let mut options = NormalizeOptions {
            languages: None,
            language_c_strs: None,
            language_ptrs: None,
            address_components: AddressComponents::default(),
            string_options: StringOptions::default(),
        };
        let ffi_options = unsafe { ffi::libpostal_get_default_options() };
        let languages: Vec<&str> = unsafe {
            Vec::from_raw_parts(
                ffi_options.languages,
                ffi_options.num_languages,
                ffi_options.num_languages,
            )
            .into_iter()
            .map(|ptr| unsafe { CStr::from_ptr(ptr).to_str().unwrap() })
            .collect()
        };
        if languages.len() == 0 {
            options.languages = None;
        } else {
            options.languages = Some(languages);
        }
        options
            .string_options
            .set(StringOptions::LATIN_ASCII, ffi_options.latin_ascii);
        options
            .string_options
            .set(StringOptions::TRANSLITERATE, ffi_options.transliterate);
        options
            .string_options
            .set(StringOptions::STRIP_ACCENTS, ffi_options.strip_accents);
        options
            .string_options
            .set(StringOptions::DECOMPOSE, ffi_options.decompose);
        options
            .string_options
            .set(StringOptions::LOWERCASE, ffi_options.lowercase);
        options
            .string_options
            .set(StringOptions::TRIM_STRING, ffi_options.trim_string);
        options.string_options.set(
            StringOptions::DROP_PARENTHETICALS,
            ffi_options.drop_parentheticals,
        );
        options.string_options.set(
            StringOptions::REPLACE_NUMERIC_HYPHENS,
            ffi_options.replace_numeric_hyphens,
        );
        options.string_options.set(
            StringOptions::DELETE_NUMERIC_HYPHENS,
            ffi_options.delete_numeric_hyphens,
        );
        options.string_options.set(
            StringOptions::SPLIT_ALPHA_FROM_NUMERIC,
            ffi_options.split_alpha_from_numeric,
        );
        options.string_options.set(
            StringOptions::REPLACE_WORD_HYPHENS,
            ffi_options.replace_word_hyphens,
        );
        options.string_options.set(
            StringOptions::DELETE_WORD_HYPHENS,
            ffi_options.delete_word_hyphens,
        );
        options.string_options.set(
            StringOptions::DELETE_FINAL_PERIODS,
            ffi_options.delete_final_periods,
        );
        options.string_options.set(
            StringOptions::DELETE_ACRONYM_PERIODS,
            ffi_options.delete_acronym_periods,
        );
        options.string_options.set(
            StringOptions::DROP_ENGLISH_POSSESSIVES,
            ffi_options.drop_english_possessives,
        );
        options.string_options.set(
            StringOptions::DELETE_APOSTROPHES,
            ffi_options.delete_apostrophes,
        );
        options
            .string_options
            .set(StringOptions::EXPAND_NUMEX, ffi_options.expand_numex);
        options
            .string_options
            .set(StringOptions::ROMAN_NUMERALS, ffi_options.roman_numerals);
        options
    }
}

impl<'a> NormalizeOptions<'a> {
    /// Create new instance with default options.
    ///
    /// `languages` override the respective option field, if given.
    pub fn new(languages: Option<Vec<&'a str>>) -> NormalizeOptions<'a> {
        let mut options = NormalizeOptions::default();
        if languages.is_some() {
            options.languages = languages;
        }
        options
    }

    unsafe fn as_libpostal_options(&mut self) -> ffi::libpostal_normalize_options {
        let mut options = ffi::libpostal_get_default_options();
        if let Some(langs) = self.languages.as_ref() {
            let mut cstrings = Vec::with_capacity(langs.len());
            let mut ptrs = Vec::with_capacity(langs.len());
            for language in langs {
                let cstring = CString::new(*language).unwrap();
                ptrs.push(cstring.as_ptr());
                cstrings.push(cstring);
            }
            self.language_c_strs = Some(cstrings);
            self.language_ptrs = Some(ptrs);
            options.languages = self.language_ptrs.as_mut().unwrap().as_mut_ptr();
            options.num_languages = langs.len() as libc::size_t;
        };
        options.address_components = self.address_components.bits;
        options.latin_ascii = self.string_options.contains(StringOptions::LATIN_ASCII);
        options.transliterate = self.string_options.contains(StringOptions::TRANSLITERATE);
        options.strip_accents = self.string_options.contains(StringOptions::STRIP_ACCENTS);
        options.decompose = self.string_options.contains(StringOptions::DECOMPOSE);
        options.lowercase = self.string_options.contains(StringOptions::LOWERCASE);
        options.trim_string = self.string_options.contains(StringOptions::TRIM_STRING);
        options.drop_parentheticals = self
            .string_options
            .contains(StringOptions::DROP_PARENTHETICALS);
        options.replace_numeric_hyphens = self
            .string_options
            .contains(StringOptions::REPLACE_NUMERIC_HYPHENS);
        options.delete_numeric_hyphens = self
            .string_options
            .contains(StringOptions::DELETE_NUMERIC_HYPHENS);
        options.split_alpha_from_numeric = self
            .string_options
            .contains(StringOptions::SPLIT_ALPHA_FROM_NUMERIC);
        options.replace_word_hyphens = self
            .string_options
            .contains(StringOptions::REPLACE_WORD_HYPHENS);
        options.delete_word_hyphens = self
            .string_options
            .contains(StringOptions::DELETE_WORD_HYPHENS);
        options.delete_final_periods = self
            .string_options
            .contains(StringOptions::DELETE_FINAL_PERIODS);
        options.delete_acronym_periods = self
            .string_options
            .contains(StringOptions::DELETE_ACRONYM_PERIODS);
        options.drop_english_possessives = self
            .string_options
            .contains(StringOptions::DROP_ENGLISH_POSSESSIVES);
        options.delete_apostrophes = self
            .string_options
            .contains(StringOptions::DELETE_APOSTROPHES);
        options.expand_numex = self.string_options.contains(StringOptions::EXPAND_NUMEX);
        options.roman_numerals = self.string_options.contains(StringOptions::ROMAN_NUMERALS);
        options
    }
}

/// Normalize address with default options.
pub fn expand_address(address: &str) -> Vec<String> {
    expand_address_with_options(address, None)
}

/// Normalize address with optional user-defined languages.
pub fn expand_address_with_options(address: &str, languages: Option<Vec<&str>>) -> Vec<String> {
    let address = CString::new(address).unwrap();
    let mut expanded: Vec<String> = Vec::new();
    let mut rust_options: NormalizeOptions;

    unsafe {
        let options: ffi::libpostal_normalize_options;
        if languages.is_some() {
            rust_options = NormalizeOptions::new(languages);
            options = rust_options.as_libpostal_options();
        } else {
            options = ffi::libpostal_get_default_options();
        }
        let mut n: libc::size_t = 0;
        let n = &mut n as *mut libc::size_t;
        let raw =
            ffi::libpostal_expand_address(address.as_ptr() as *const libc::c_char, options, n);
        for i in 0..*n {
            if let Some(phrase) = raw.add(i).as_ref() {
                let normalized = CStr::from_ptr(*phrase);
                expanded.push(String::from(normalized.to_str().unwrap()));
            };
        }
        ffi::libpostal_expansion_array_destroy(raw, *n);
    }
    expanded
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn toggle_address_components_default() {
        let components: AddressComponents = Default::default();
        assert_eq!(components.bits, 0b1100000011111110);
        assert_eq!(components, Default::default());
    }

    #[test]
    fn toggle_address_components_all() {
        let mut components = AddressComponents::NONE;
        components.toggle(AddressComponents::all());
        assert_eq!(components.bits, AddressComponents::all().bits);
        components.toggle(AddressComponents::all());
        assert_eq!(components.bits, AddressComponents::NONE.bits);
    }

    #[test]
    fn normalized_options_new() {
        unsafe {
            ffi::libpostal_setup();
            ffi::libpostal_setup_language_classifier();
        }
        let options = NormalizeOptions::new(None);
        assert_eq!(options.languages, None);
        assert_eq!(options.address_components, Default::default());
        unsafe {
            ffi::libpostal_teardown_language_classifier();
            ffi::libpostal_teardown();
        }
    }

    #[test]
    fn normalized_options_new_with_languages() {
        unsafe {
            ffi::libpostal_setup();
            ffi::libpostal_setup_language_classifier();
        }
        let languages = vec!["en", "gr"];
        let mut options = NormalizeOptions::new(Some(languages.clone()));
        let ffi_options: ffi::libpostal_normalize_options;
        unsafe {
            ffi_options = options.as_libpostal_options();
            let mut ffi_languages = vec![];
            for i in 0..ffi_options.num_languages {
                ffi_languages.push(
                    CStr::from_ptr(*ffi_options.languages.add(i))
                        .to_str()
                        .unwrap(),
                );
            }
            assert_eq!(languages, ffi_languages);
        }
        unsafe {
            ffi::libpostal_teardown_language_classifier();
            ffi::libpostal_teardown();
        }
    }
}
