//! Normalization utilities.
//!
//! # Examples
//!
//! ```
//! use std::ffi::NulError;
//!
//! use rustpostal::{expand, LibModules};
//! use rustpostal::error::RuntimeError;
//!
//! fn main() -> Result<(), RuntimeError> {
//!     let postal_module = LibModules::Expand;
//!     postal_module.setup()?;
//!
//!     let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
//!
//!     let languages = ["en", "gb"];
//!     let expanded = expand::expand_address_with_options(address, Some(languages.iter()))?;
//!
//!     for variation in &expanded {
//!         println!("{}", variation);
//!     }
//!
//!     Ok(())
//! }
//! ```
#![allow(unused)]

use bitflags::bitflags;
use std::ffi::{CStr, CString, NulError};
use std::iter::Iterator;

use libc::{c_char, size_t};

use crate::ffi;

bitflags! {
    /// Bit set of active string options.
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
#[derive(Clone, Hash, Debug)]
struct LibpostalNormalizeOptions {
    ffi: Option<ffi::libpostal_normalize_options>,
    lang_buffer: Option<Vec<*const c_char>>,
}

/// Normalization options.
///
/// Options are required to expand a postal address to its normalized variations. They are created
/// by defining optionally language-codes for normalization (e.g. 'en'), and then gradually
/// adding more options.
///
/// A `expand` method is implemented to use the options for normalizing an address.
///
/// # Examples
///
/// ```
/// use rustpostal::expand::{AddressComponents, StringOptions, NormalizeOptions};
///
/// let languages = ["en", "gb"];
/// let mut options = NormalizeOptions::new(Some(languages.iter()));
/// assert_eq!(options.languages(), Some(&languages[..]));
///
/// let s_options = StringOptions::TRANSLITERATE | StringOptions::LOWERCASE;
/// let components = AddressComponents::NAME | AddressComponents::LEVEL;
///
/// options.add_string_option(s_options);
/// assert!(options.string_options().as_ref().unwrap().contains(s_options));
///
/// options.add_address_component(components);
/// assert!(options.address_components().as_ref().unwrap().contains(components));
#[derive(Clone, Hash, Debug)]
pub struct NormalizeOptions<'a> {
    languages: Option<Vec<&'a str>>,
    address_components: Option<AddressComponents>,
    string_options: Option<StringOptions>,
    libpostal_options: LibpostalNormalizeOptions,
}

/// Collections of normalized variations of postal address.
#[derive(Clone, Hash, Debug)]
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
    fn update_languages<'a>(&mut self, languages: &[&'a str]) {
        if self.lang_buffer.is_some() {
            return;
        }
        self.free_lang_ptrs();
        let mut lang_buffer: Vec<*const c_char> = languages
            .iter()
            .map(|&s| CString::new(s).unwrap().into_raw() as *const c_char)
            .collect();
        let ffi = self.inner_mut();
        ffi.languages = lang_buffer.as_mut_ptr();
        ffi.num_languages = lang_buffer.len();
        self.lang_buffer = Some(lang_buffer);
    }

    /// Normalize address.
    fn expand(&mut self, address: &CStr) -> NormalizedAddress {
        let mut result: NormalizedAddress = Default::default();
        let options = self.ffi.take().unwrap();
        let raw =
            unsafe { ffi::libpostal_expand_address(address.as_ptr(), options, &mut result.n) };
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

impl<'a> Default for NormalizeOptions<'a> {
    fn default() -> Self {
        NormalizeOptions {
            languages: Default::default(),
            address_components: Default::default(),
            string_options: Default::default(),
            libpostal_options: Default::default(),
        }
    }
}

impl<'a> NormalizeOptions<'a> {
    /// Create new instance with default options.
    ///
    /// `languages` override the respective option field, if given.
    pub fn new<'b, T>(languages: Option<T>) -> NormalizeOptions<'a>
    where
        'a: 'b,
        T: Iterator<Item = &'b &'a str>,
    {
        let mut options = NormalizeOptions::default();
        if let Some(languages) = languages {
            options.languages = Some(languages.map(|&s| s).collect());
        }
        options
    }

    /// Add string option.
    pub fn add_string_option(&mut self, option: StringOptions) {
        if let Some(options) = &mut self.string_options {
            options.insert(option);
        } else {
            self.string_options = Some(option);
        }
    }

    /// Add address component option.
    pub fn add_address_component(&mut self, component: AddressComponents) {
        if let Some(components) = &mut self.address_components {
            components.insert(component);
        } else {
            self.address_components = Some(component);
        }
    }

    /// Create libpostal options.
    fn libpostal_options(&self) -> LibpostalNormalizeOptions {
        let mut options: LibpostalNormalizeOptions = Default::default();
        if let Some(string_options) = &self.string_options {
            options.update_string_options(string_options);
        }
        if let Some(address_components) = &self.address_components {
            options.update_address_components(address_components);
        }
        if let Some(languages) = &self.languages {
            options.update_languages(languages.as_slice());
        }
        options
    }

    /// Return current languages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustpostal::expand::NormalizeOptions;
    ///
    /// let options = NormalizeOptions::default();
    /// assert_eq!(options.languages(), None);
    ///
    /// let languages = ["en", "gb"];
    /// let options = NormalizeOptions::new(Some(languages.iter()));
    /// assert_eq!(options.languages(), Some(&languages[..]));
    /// ```
    pub fn languages(&self) -> Option<&[&str]> {
        if let Some(languages) = &self.languages {
            return Some(languages.as_slice());
        }
        None
    }

    /// Return current address components.
    ///
    /// ```
    /// use rustpostal::expand::{NormalizeOptions, AddressComponents};
    ///
    /// let mut options = NormalizeOptions::default();
    /// assert_eq!(options.address_components(), None);
    /// options.add_address_component(AddressComponents::NAME);
    /// assert_eq!(options.address_components(), Some(&AddressComponents::NAME));
    /// ```
    pub fn address_components(&self) -> Option<&AddressComponents> {
        self.address_components.as_ref()
    }

    /// Return current string options.
    ///
    /// ```
    /// use rustpostal::expand::{NormalizeOptions, StringOptions};
    ///
    /// let mut options = NormalizeOptions::default();
    /// assert_eq!(options.string_options(), None);
    /// options.add_string_option(StringOptions::TRANSLITERATE);
    /// assert_eq!(options.string_options(), Some(&StringOptions::TRANSLITERATE));
    /// ```
    pub fn string_options(&self) -> Option<&StringOptions> {
        self.string_options.as_ref()
    }

    /// Expand address into normalized variations using libpostal.
    ///
    /// ## Examples
    ///
    /// ```
    /// use std::ffi::NulError;
    ///
    /// use rustpostal::LibModules;
    /// use rustpostal::expand::NormalizeOptions;
    /// use rustpostal::error::RuntimeError;
    ///
    /// fn main() -> Result<(), RuntimeError> {
    ///     let postal_module = LibModules::Expand;
    ///     postal_module.setup()?;
    ///
    ///     let mut options = NormalizeOptions::default();
    ///     let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
    ///
    ///     let expanded = options.expand(address)?;
    ///     for variation in &expanded {
    ///         assert!(variation.ends_with("kingdom"))
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Errors
    ///
    /// The method will return an error if the supplied address
    /// contains an internal null byte. The error is represented by
    /// [`NulError`](https://doc.rust-lang.org/nightly/std/ffi/c_str/struct.NulError.html).
    pub fn expand(&mut self, address: &str) -> Result<NormalizedAddress, NulError> {
        let mut options = self.libpostal_options();
        let c_address = CString::new(address)?;
        Ok(options.expand(&c_address))
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

impl NormalizedAddress {
    /// Returns an iterator over the variations
    /// of the normalized address.
    pub fn iter(&self) -> std::slice::Iter<String> {
        self.variations.as_slice().iter()
    }

    /// Returns an iterator that allows modifying variations
    /// of the normalized address.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<String> {
        self.variations.as_mut_slice().iter_mut()
    }
}

impl<'a> IntoIterator for &'a NormalizedAddress {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut NormalizedAddress {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Normalize address with default options.
///
/// ## Errors
///
/// The method will return an error if the supplied address
/// contains an internal null byte. The error is represented by
/// [`NulError`](https://doc.rust-lang.org/nightly/std/ffi/c_str/struct.NulError.html).
pub fn expand_address<'a>(address: &'a str) -> Result<NormalizedAddress, NulError> {
    let mut options = NormalizeOptions::default();
    options.expand(address)
}

/// Normalize address with optional user-defined languages.
///
/// ## Errors
///
/// The method will return an error if the supplied address
/// contains an internal null byte. The error is represented by
/// [`NulError`](https://doc.rust-lang.org/nightly/std/ffi/c_str/struct.NulError.html).
pub fn expand_address_with_options<'a, 'b, T>(
    address: &'a str,
    languages: Option<T>,
) -> Result<NormalizedAddress, NulError>
where
    'a: 'b,
    T: Iterator<Item = &'b &'a str>,
{
    let mut options = NormalizeOptions::new(languages);
    options.expand(address)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::RuntimeError;
    use crate::LibModules;

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
        options.update_languages(&languages);
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

    #[test]
    fn libpostal_normalize_options_expand() -> Result<(), RuntimeError> {
        let postal_module = LibModules::Expand;
        postal_module.setup()?;

        let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
        let c_address = CString::new(address)?;

        let mut libpostal_options: LibpostalNormalizeOptions = Default::default();

        let expanded = libpostal_options.expand(&c_address);

        assert!(expanded.n > 0);
        for variation in &expanded.variations {
            assert!(variation.ends_with("kingdom"));
        }

        for variation in &expanded {
            assert!(variation.ends_with("kingdom"));
        }
        Ok(())
    }

    #[test]
    fn normalized_address_iter() {
        let mut normalized = NormalizedAddress::default();
        normalized.variations.push(String::from("wat"));
        normalized.variations.push(String::from("what"));
        let mut iterator = normalized.iter();

        assert_eq!(iterator.next(), Some(&String::from("wat")));
        assert_eq!(iterator.next(), Some(&String::from("what")));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn normalized_address_iter_mut() {
        let mut normalized = NormalizedAddress::default();
        normalized.variations.push(String::from("wat"));
        normalized.variations.push(String::from("what"));

        for variation in normalized.iter_mut() {
            variation.push_str(" else");
        }

        let mut iterator = normalized.iter_mut();
        assert_eq!(iterator.next(), Some(&mut String::from("wat else")));
        assert_eq!(iterator.next(), Some(&mut String::from("what else")));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn normalized_address_mut_into_iter() {
        let mut normalized = NormalizedAddress::default();
        let variations = ["wat", "what"];
        for variation in &variations {
            normalized.variations.push(String::from(*variation))
        }
        for variation in &mut normalized {
            variation.push_str(" else");
        }

        let expected = ["wat else", "what else"];
        for (i, variation) in normalized.iter().enumerate() {
            assert_eq!(variation.as_str(), expected[i]);
        }
    }
}
