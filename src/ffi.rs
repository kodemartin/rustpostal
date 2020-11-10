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
        n: *const libc::size_t,
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
pub struct libpostal_normalize_options {
    languages: *const *const libc::c_char,
    num_lanugages: libc::size_t,
    address_components: u16,
    // String options
    latin_ascii: bool,
    transliterate: bool,
    strip_accents: bool,
    decompose: bool,
    lowercase: bool,
    trim_string: bool,
    drop_parentheticals: bool,
    replace_numeric_hyphens: bool,
    delete_numeric_hyphens: bool,
    split_alpha_from_numeric: bool,
    replace_word_hyphens: bool,
    delete_word_hyphens: bool,
    delete_final_periods: bool,
    delete_acronym_periods: bool,
    drop_english_possessives: bool,
    delete_apostrophes: bool,
    expand_numex: bool,
    roman_numerals: bool,
}
