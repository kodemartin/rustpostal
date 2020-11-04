use std::ffi::{CStr, CString};
use std::process;

mod ffi;

pub struct AddressParserResponse {
    pub components: Vec<String>,
    pub labels: Vec<String>,
}

impl AddressParserResponse {
    pub fn new() -> AddressParserResponse {
        AddressParserResponse {
            components: Vec::new(),
            labels: Vec::new(),
        }
    }
}

impl IntoIterator for AddressParserResponse {
    type Item = (String, String);
    type IntoIter = std::iter::Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter().zip(self.labels)
    }
}

pub struct AddressParserOptions<'a> {
    language: Option<&'a str>,
    country: Option<&'a str>,
}

impl<'a> AddressParserOptions<'a> {
    pub fn new (language: Option<&'a str>, country: Option<&'a str>) -> AddressParserOptions<'a> {
        let (mut default_l, mut default_c) = (None, None);
        if language.is_none() || country.is_none() {
            let (l, c) = Self::get_default_options();
            default_l = l;
            default_c = c;
        }
        let language = language.or(default_l);
        let country = country.or(default_c);
        AddressParserOptions{ language, country }
    }

    fn get_default_options() -> (Option<&'a str>, Option<&'a str>) {
        let (mut language, mut country) = (None, None);
        unsafe {
            let options = ffi::libpostal_get_address_parser_default_options();
            if !options.language.is_null() {
                language = Some(CStr::from_ptr(options.language).to_str().unwrap());
            }
            if !options.country.is_null() {
                country = Some(CStr::from_ptr(options.language).to_str().unwrap());
            }
        }
        (language, country)
    }

    unsafe fn as_ffi_options(&self) -> ffi::libpostal_address_parser_options {
        let (language, country): (*const libc::c_char, *const libc::c_char);
        if self.language.is_none() {
            language = std::ptr::null();
        } else {
            language = self.language.unwrap().as_ptr() as *const libc::c_char;
        }
        if self.country.is_none() {
            country = std::ptr::null();
        } else {
            country = self.country.unwrap().as_ptr() as *const libc::c_char;
        }
        ffi::libpostal_address_parser_options { language, country }
    }
}

pub unsafe fn setup() {
    if !ffi::libpostal_setup() || !ffi::libpostal_setup_parser() {
        process::exit(1);
    }
}

pub unsafe fn teardown() {
    ffi::libpostal_teardown();
    ffi::libpostal_teardown_parser();
}

pub fn parse_address(address: &str, language: Option<&str>, country: Option<&str>) -> AddressParserResponse {
    let address = CString::new(address).unwrap();
    let mut response = AddressParserResponse::new();

    let options = AddressParserOptions::new(language, country);

    unsafe {
        let options = options.as_ffi_options();
        let raw = ffi::libpostal_parse_address(address.as_ptr() as *const libc::c_char, options);
        if let Some(parsed) = raw.as_ref() {
            for i in 0..parsed.num_components {
                let component = CStr::from_ptr(*parsed.components.add(i));
                let label = CStr::from_ptr(*parsed.labels.add(i));
                response
                    .components
                    .push(String::from(component.to_str().unwrap()));
                response.labels.push(String::from(label.to_str().unwrap()));
            }
        };
        ffi::libpostal_address_parser_response_destroy(raw);
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_address_parser_options() {
        let options = AddressParserOptions::new(None, None);
        assert_eq!(options.language, None);
        assert_eq!(options.country, None);
        let options = AddressParserOptions::new(None, Some("EN"));
        assert_eq!(options.language, None);
        assert_eq!(options.country, Some("EN"));
    }

}
