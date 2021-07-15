#![allow(unused)]

use libc;

#[link(name = "postal")]
extern "C" {
    pub fn libpostal_get_address_parser_default_options() -> libpostal_address_parser_options;
    pub fn libpostal_setup() -> bool;
    pub fn libpostal_setup_parser() -> bool;
    pub fn libpostal_setup_language_classifier() -> bool;
    pub fn libpostal_teardown() -> bool;
    pub fn libpostal_teardown_parser() -> bool;
    pub fn libpostal_teardown_language_classifier() -> bool;

    pub fn libpostal_parse_address(
        address: *const libc::c_char,
        options: libpostal_address_parser_options,
    ) -> *const libpostal_address_parser_response;

    pub fn libpostal_address_parser_response_destroy(
        response: *const libpostal_address_parser_response,
    );

    pub fn libpostal_get_default_options() -> libpostal_normalize_options;
    pub fn libpostal_expand_address(
        input: *const libc::c_char,
        options: libpostal_normalize_options,
        n: *mut libc::size_t,
    ) -> *const *const libc::c_char;
    pub fn libpostal_expansion_array_destroy(
        expansions: *const *const libc::c_char,
        n: libc::size_t,
    );
}

#[repr(C)]
pub struct libpostal_address_parser_options {
    pub language: *const libc::c_char,
    pub country: *const libc::c_char,
}

#[repr(C)]
pub struct libpostal_address_parser_response {
    pub num_components: usize,
    pub components: *mut *const libc::c_char,
    pub labels: *mut *const libc::c_char,
}

#[repr(C)]
#[derive(Clone, Hash, Debug)]
pub struct libpostal_normalize_options {
    pub languages: *mut *const libc::c_char,
    pub num_languages: libc::size_t,
    pub address_components: u16,
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
