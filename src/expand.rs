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
use std::iter::Iterator;

use libc::{c_char, size_t};

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
    #[derive(Default)]
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

/// Wrap the options to pass to the C library.
struct LibpostalNormalizeOptions {
    ffi: Option<ffi::libpostal_normalize_options>,
    lang_buffer: Option<Vec<*const c_char>>,
}

/// Normalization options.
pub struct NormalizeOptions<'a, T: Iterator<Item = &'a str>> {
    pub languages: Option<T>,
    pub address_components: AddressComponents,
    pub string_options: StringOptions,
    libpostal_options: LibpostalNormalizeOptions,
}

/// Collections of normalized variations of postal address.
pub struct NormalizedAddress {
    variations: Vec<String>,
    n: size_t,
}

impl LibpostalNormalizeOptions {
    /// Access the inner ffi options
    fn inner_mut(&mut self) -> &mut ffi::libpostal_normalize_options {
        self.ffi
            .get_or_insert(unsafe { ffi::libpostal_get_default_options() })
    }

    /// Free pointers to language options.
    fn free_lang_ptrs(&mut self) {
        let c_strings_buffered = self.lang_buffer.as_mut().is_some();
        let ffi = self.inner_mut();
        unsafe {
            for i in 0..ffi.num_languages {
                let ptr = ffi.languages.add(i);
                if (c_strings_buffered) {
                    let cstring = CString::from_raw(*ptr as *mut c_char);
                } else {
                    libc::free(*ptr as *mut libc::c_void);
                }
            }
        }
    }

    /// Update string options in ffi.
    fn update_string_options(&mut self, string_options: &StringOptions) {
        let (src, dst) = (self.inner_mut(), string_options);
        src.latin_ascii = dst.contains(StringOptions::LATIN_ASCII);
        src.transliterate = dst.contains(StringOptions::TRANSLITERATE);
        src.strip_accents = dst.contains(StringOptions::STRIP_ACCENTS);
        src.decompose = dst.contains(StringOptions::DECOMPOSE);
        src.lowercase = dst.contains(StringOptions::LOWERCASE);
        src.trim_string = dst.contains(StringOptions::TRIM_STRING);
        src.drop_parentheticals = dst.contains(StringOptions::DROP_PARENTHETICALS);
        src.replace_numeric_hyphens = dst.contains(StringOptions::REPLACE_NUMERIC_HYPHENS);
        src.delete_numeric_hyphens = dst.contains(StringOptions::DELETE_NUMERIC_HYPHENS);
        src.split_alpha_from_numeric = dst.contains(StringOptions::SPLIT_ALPHA_FROM_NUMERIC);
        src.replace_word_hyphens = dst.contains(StringOptions::REPLACE_WORD_HYPHENS);
        src.delete_word_hyphens = dst.contains(StringOptions::DELETE_WORD_HYPHENS);
        src.delete_final_periods = dst.contains(StringOptions::DELETE_FINAL_PERIODS);
        src.delete_acronym_periods = dst.contains(StringOptions::DELETE_ACRONYM_PERIODS);
        src.drop_english_possessives = dst.contains(StringOptions::DROP_ENGLISH_POSSESSIVES);
        src.delete_apostrophes = dst.contains(StringOptions::DELETE_APOSTROPHES);
        src.expand_numex = dst.contains(StringOptions::EXPAND_NUMEX);
        src.roman_numerals = dst.contains(StringOptions::ROMAN_NUMERALS);
    }

    /// Update address components in ffi.
    fn update_address_components(&mut self, address_components: &AddressComponents) {
        self.inner_mut().address_components = address_components.bits;
    }

    /// Update languages in ffi.
    fn update_languages<'a, T: Iterator<Item = &'a str>>(&mut self, languages: &mut T) {
        if self.lang_buffer.is_some() {
            return;
        }
        self.free_lang_ptrs();
        let mut lang_buffer: Vec<*const c_char> = languages
            .by_ref()
            .map(|s| CString::new(s).unwrap().into_raw() as *const c_char)
            .collect();
        let ffi = self.inner_mut();
        ffi.languages = lang_buffer.as_mut_ptr();
        ffi.num_languages = lang_buffer.len();
        self.lang_buffer = Some(lang_buffer);
    }

    /// Normalize address.
    fn expand(&mut self, address: &str) -> NormalizedAddress {
        let address = address.as_ptr() as *const c_char;
        let mut result: NormalizedAddress = Default::default();
        let options = self.ffi.take().unwrap();
        let raw = unsafe { ffi::libpostal_expand_address(address, options, &mut result.n) };
        result.variations = Vec::with_capacity(result.n);
        unsafe {
            for i in 0..result.n {
                if let Some(phrase) = raw.add(i).as_ref() {
                    let variation = CStr::from_ptr(*phrase);
                    result
                        .variations
                        .push(String::from(variation.to_str().unwrap()));
                };
            }
            ffi::libpostal_expansion_array_destroy(raw, result.n);
        }
        result
    }
}

impl Default for LibpostalNormalizeOptions {
    fn default() -> Self {
        LibpostalNormalizeOptions {
            ffi: Some(unsafe { ffi::libpostal_get_default_options() }),
            lang_buffer: Default::default(),
        }
    }
}

impl Drop for LibpostalNormalizeOptions {
    fn drop(&mut self) {
        self.free_lang_ptrs();
    }
}

impl<'a, T> Default for NormalizeOptions<'a, T>
where
    T: Iterator<Item = &'a str>,
{
    fn default() -> Self {
        NormalizeOptions {
            languages: Default::default(),
            address_components: Default::default(),
            string_options: Default::default(),
            libpostal_options: Default::default(),
        }
    }
}

impl<'a, T: Iterator<Item = &'a str>> NormalizeOptions<'a, T> {
    /// Create new instance with default options.
    ///
    /// `languages` override the respective option field, if given.
    pub fn new(languages: Option<T>) -> NormalizeOptions<'a, T> {
        let mut options = NormalizeOptions::default();
        if languages.is_some() {
            options.languages = languages;
        }
        options
    }

    /// Add string option.
    pub fn add_string_option(&mut self, option: StringOptions) {
        self.string_options.insert(option);
    }

    /// Add address component option.
    pub fn add_address_component(&mut self, component: AddressComponents) {
        self.address_components.insert(component);
    }

    /// Create libpostal options.
    fn libpostal_options(&mut self) -> LibpostalNormalizeOptions {
        let mut options: LibpostalNormalizeOptions = Default::default();
        options.update_string_options(&self.string_options);
        options.update_address_components(&self.address_components);
        if let Some(languages) = &mut self.languages {
            options.update_languages(languages);
        }
        options
    }

    /// Expand address.
    pub fn expand(&mut self, address: &str) -> NormalizedAddress {
        let mut options = self.libpostal_options();
        options.expand(address)
    }
}

impl Default for NormalizedAddress {
    fn default() -> Self {
        NormalizedAddress {
            variations: Vec::new(),
            n: 0,
        }
    }
}

/// Normalize address with default options.
/// pub fn expand_address<'a, T>(address: &'a str) -> T
///     where T: Iterator<Item = &'a str>
/// {
///     expand_address_with_options(address, None)
/// }


/// Normalize address with optional user-defined languages.
/// pub fn expand_address_with_options<'a, T>(address: &'a str, languages: Option<T>) -> T 
///     where T: Iterator<Item=&'a str>
/// {
///     let address = CString::new(address).unwrap();
///     let mut rust_options = NormalizeOptions::new(languages);
///     let mut n: libc::size_t = 0;
/// 
///     unsafe {
///         let options = rust_options.as_libpostal_options();
///         let raw =
///             ffi::libpostal_expand_address(address.as_ptr() as *const libc::c_char, options, &mut n);
///         let mut expanded = Vec::with_capacity(n);
///         for i in 0..n {
///             if let Some(phrase) = raw.add(i).as_ref() {
///                 let normalized = CStr::from_ptr(*phrase);
///                 expanded.push(String::from(normalized.to_str().unwrap()));
///             };
///         }
///         ffi::libpostal_expansion_array_destroy(raw, n);
///         expanded
///     }
/// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_libpostal_normalize_options() {
        let mut options: LibpostalNormalizeOptions = Default::default();
        assert!(options.ffi.is_some());
        assert_eq!(*(&options.ffi.as_ref().unwrap().num_languages), 0);
        assert!(options.lang_buffer.is_none());
    }

    #[test]
    fn libpostal_normalize_options_update_languages() {
        let mut languages = ["en", "gr"];
        let mut options: LibpostalNormalizeOptions = Default::default();
        options.update_languages(&mut languages.iter().by_ref().map(|l| *l));
        let ffi = &options.ffi.as_ref().unwrap();
        for i in 0..ffi.num_languages {
            unsafe {
                let ptr = ffi.languages.add(i);
                let cstr = CStr::from_ptr(*ptr);
                assert_eq!(cstr.to_str(), Ok(languages[i]));
            }
        }
    }

    #[test]
    fn libpostal_normalize_options_update_string_options() {
        let mut options: LibpostalNormalizeOptions = Default::default();
        let s_options = StringOptions::TRANSLITERATE | StringOptions::LOWERCASE;
        options.update_string_options(&s_options);
        let ffi = &options.ffi.as_ref().unwrap();
        assert!(ffi.transliterate);
        assert!(ffi.lowercase);
        assert!(!ffi.latin_ascii);
    }

    #[test]
    fn libpostal_normalize_options_update_address_components() {
        let mut options: LibpostalNormalizeOptions = Default::default();
        let components = AddressComponents::NAME | AddressComponents::LEVEL;
        options.update_address_components(&components);
        let ffi = &options.ffi.as_ref().unwrap();
        assert_eq!(ffi.address_components, components.bits);
    }
}
