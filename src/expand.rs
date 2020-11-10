#![allow(unused)]

use std::ffi::{CStr, CString};

use crate::ffi;

//* Bit-set for toggling address components
const LIBPOSTAL_ADDRESS_NONE: u16 = 0;
const LIBPOSTAL_ADDRESS_ANY: u16 = 1 << 0;
const LIBPOSTAL_ADDRESS_NAME: u16 = 1 << 1;
const LIBPOSTAL_ADDRESS_HOUSE_NUMBER: u16 = 1 << 2;
const LIBPOSTAL_ADDRESS_STREET: u16 = 1 << 3;
const LIBPOSTAL_ADDRESS_UNIT: u16 = 1 << 4;
const LIBPOSTAL_ADDRESS_LEVEL: u16 = 1 << 5;
const LIBPOSTAL_ADDRESS_STAIRCASE: u16 = 1 << 6;
const LIBPOSTAL_ADDRESS_ENTRANCE: u16 = 1 << 7;
const LIBPOSTAL_ADDRESS_CATEGORY: u16 = 1 << 8;
const LIBPOSTAL_ADDRESS_NEAR: u16 = 1 << 9;
const LIBPOSTAL_ADDRESS_TOPONYM: u16 = 1 << 13;
const LIBPOSTAL_ADDRESS_POSTAL_CODE: u16 = 1 << 14;
const LIBPOSTAL_ADDRESS_PO_BOX: u16 = 1 << 15;
const LIBPOSTAL_ADDRESS_ALL: u16 = 1 << 16 - 1;
const LIBPOSTAL_ADDRESS_DEFAULT_COMPONENTS: u16 = LIBPOSTAL_ADDRESS_NAME
    | LIBPOSTAL_ADDRESS_HOUSE_NUMBER
    | LIBPOSTAL_ADDRESS_STREET
    | LIBPOSTAL_ADDRESS_PO_BOX
    | LIBPOSTAL_ADDRESS_UNIT
    | LIBPOSTAL_ADDRESS_LEVEL
    | LIBPOSTAL_ADDRESS_ENTRANCE
    | LIBPOSTAL_ADDRESS_STAIRCASE
    | LIBPOSTAL_ADDRESS_POSTAL_CODE;

#[derive(Debug)]
pub struct AddressComponents(u16);

impl AddressComponents {
    pub fn new() -> AddressComponents {
        AddressComponents(LIBPOSTAL_ADDRESS_DEFAULT_COMPONENTS)
    }

    pub fn toggle_name(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_NAME;
    }

    pub fn toggle_house_number(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_HOUSE_NUMBER;
    }

    pub fn toggle_street(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_STREET;
    }

    pub fn toggle_unit(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_UNIT;
    }

    pub fn toggle_level(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_LEVEL;
    }

    pub fn toggle_staircase(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_STAIRCASE;
    }

    pub fn toggle_entrance(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_ENTRANCE;
    }

    pub fn toggle_category(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_CATEGORY;
    }

    pub fn toggle_near(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_NEAR;
    }

    pub fn toggle_toponym(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_TOPONYM;
    }

    pub fn toggle_postal_code(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_POSTAL_CODE;
    }

    pub fn toggle_po_box(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_PO_BOX;
    }

    pub fn toggle_all(&mut self) {
        self.0 ^= LIBPOSTAL_ADDRESS_ALL;
    }
}

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
    pub fn new(languages: Option<Vec<&'a str>>) -> NormalizeOptions<'a> {
        let mut options = Self::get_default_options();
        if languages.is_some() {
            options.languages = languages;
        }
        options
    }

    pub fn get_default_options() -> NormalizeOptions<'a> {
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
            address_components: AddressComponents::new(),
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
        options.address_components = self.address_components.0;
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

pub fn expand_address(address: &str) -> Vec<String> {
    expand_address_with_options(address, None)
}

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
    fn toggle_address_components_new() {
        let components = AddressComponents::new();
        assert_eq!(components.0, LIBPOSTAL_ADDRESS_DEFAULT_COMPONENTS);
    }

    #[test]
    fn toggle_address_components_all() {
        let mut components = AddressComponents::new();
        components.0 = LIBPOSTAL_ADDRESS_NONE;
        components.toggle_all();
        assert_eq!(components.0, LIBPOSTAL_ADDRESS_ALL);
        components.toggle_all();
        assert_eq!(components.0, LIBPOSTAL_ADDRESS_NONE);
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
            options.address_components.0,
            LIBPOSTAL_ADDRESS_DEFAULT_COMPONENTS
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
