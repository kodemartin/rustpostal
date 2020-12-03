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

use std::ffi::{CStr, CString};
use bitflags::bitflags;

use crate::ffi;

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
#[derive(Debug)]
pub struct NormalizeOptions<'a> {
    pub languages: Option<Vec<&'a str>>,
    language_c_strs: Option<Vec<CString>>,
    language_ptrs: Option<Vec<*const libc::c_char>>,
    pub address_components: AddressComponents,
    // String options
    pub latin_ascii: bool,
    pub transliterate: bool,
    pub strip_accents: bool,
    pub decompose: bool,
    pub lowercase: bool,
    pub trim_string: bool,
    pub drop_parentheticals: bool,
    pub replace_numeric_hyphens: bool,
    pub delete_numeric_hyphens: bool,
    pub split_alpha_from_numeric: bool,
    pub replace_word_hyphens: bool,
    pub delete_word_hyphens: bool,
    pub delete_final_periods: bool,
    pub delete_acronym_periods: bool,
    pub drop_english_possessives: bool,
    pub delete_apostrophes: bool,
    pub expand_numex: bool,
    pub roman_numerals: bool,
}

impl<'a> NormalizeOptions<'a> {
    /// Create new instance with default options.
    ///
    /// `languages` override the respective option field, if given.
    pub fn new(languages: Option<Vec<&'a str>>) -> NormalizeOptions<'a> {
        let mut options = Self::get_default_options();
        if languages.is_some() {
            options.languages = languages;
        }
        options
    }

    fn get_default_options() -> NormalizeOptions<'a> {
        let mut options = Self::initialize();
        unsafe {
            let ffi_options = ffi::libpostal_get_default_options();
            if !ffi_options.languages.is_null() {
                let mut languages = vec![];
                for i in 0..ffi_options.num_languages {
                    let language = CStr::from_ptr(*ffi_options.languages.add(i));
                    languages.push(language.to_str().unwrap());
                }
                if languages.len() == 0 {
                    options.languages = None;
                } else {
                    options.languages = Some(languages);
                }
            }
            options.latin_ascii = ffi_options.latin_ascii;
            options.transliterate = ffi_options.transliterate;
            options.strip_accents = ffi_options.strip_accents;
            options.decompose = ffi_options.decompose;
            options.lowercase = ffi_options.lowercase;
            options.trim_string = ffi_options.trim_string;
            options.drop_parentheticals = ffi_options.drop_parentheticals;
            options.replace_numeric_hyphens = ffi_options.replace_numeric_hyphens;
            options.delete_numeric_hyphens = ffi_options.delete_numeric_hyphens;
            options.split_alpha_from_numeric = ffi_options.split_alpha_from_numeric;
            options.replace_word_hyphens = ffi_options.replace_word_hyphens;
            options.delete_word_hyphens = ffi_options.delete_word_hyphens;
            options.delete_final_periods = ffi_options.delete_final_periods;
            options.delete_acronym_periods = ffi_options.delete_acronym_periods;
            options.drop_english_possessives = ffi_options.drop_english_possessives;
            options.delete_apostrophes = ffi_options.delete_apostrophes;
            options.expand_numex = ffi_options.expand_numex;
            options.roman_numerals = ffi_options.roman_numerals;
        }
        options
    }

    fn initialize() -> NormalizeOptions<'a> {
        NormalizeOptions {
            languages: None,
            language_c_strs: None,
            language_ptrs: None,
            address_components: Default::default(),
            latin_ascii: false,
            transliterate: false,
            strip_accents: false,
            decompose: false,
            lowercase: false,
            trim_string: false,
            drop_parentheticals: false,
            replace_numeric_hyphens: false,
            delete_numeric_hyphens: false,
            split_alpha_from_numeric: false,
            replace_word_hyphens: false,
            delete_word_hyphens: false,
            delete_final_periods: false,
            delete_acronym_periods: false,
            drop_english_possessives: false,
            delete_apostrophes: false,
            expand_numex: false,
            roman_numerals: false,
        }
    }

    unsafe fn as_libpostal_options(&mut self) -> ffi::libpostal_normalize_options {
        let mut options = ffi::libpostal_get_default_options();
        if let Some(langs) = self.languages.as_ref() {
            self.language_c_strs = Some(vec![]);
            self.language_ptrs = Some(vec![]);
            let c_str = self.language_c_strs.as_mut().unwrap();
            let ptrs = self.language_ptrs.as_mut().unwrap();
            for (i, language) in langs.iter().enumerate() {
                c_str.push(CString::new(*language).unwrap());
                ptrs.push(c_str[i].as_ptr());
            }
            options.languages = self.language_ptrs.as_mut().unwrap().as_mut_ptr();
            options.num_languages = langs.len() as libc::size_t;
        };
        options.address_components = self.address_components.bits;
        options.latin_ascii = self.latin_ascii;
        options.transliterate = self.transliterate;
        options.strip_accents = self.strip_accents;
        options.decompose = self.decompose;
        options.lowercase = self.lowercase;
        options.trim_string = self.trim_string;
        options.drop_parentheticals = self.drop_parentheticals;
        options.replace_numeric_hyphens = self.replace_numeric_hyphens;
        options.delete_numeric_hyphens = self.delete_numeric_hyphens;
        options.split_alpha_from_numeric = self.split_alpha_from_numeric;
        options.replace_word_hyphens = self.replace_word_hyphens;
        options.delete_word_hyphens = self.delete_word_hyphens;
        options.delete_final_periods = self.delete_final_periods;
        options.delete_acronym_periods = self.delete_acronym_periods;
        options.drop_english_possessives = self.drop_english_possessives;
        options.delete_apostrophes = self.delete_apostrophes;
        options.expand_numex = self.expand_numex;
        options.roman_numerals = self.roman_numerals;
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
        assert_eq!(
            options.address_components,
            Default::default()
        );
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
