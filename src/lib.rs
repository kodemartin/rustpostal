use std::ffi::{CStr, CString};
use std::process;

mod ffi;

pub struct AddressParserResponse {
    pub components: Vec<String>,
    pub labels: Vec<String>,
}

impl IntoIterator for AddressParserResponse {
    type Item = (String, String);
    type IntoIter = std::iter::Zip<std::vec::IntoIter<String>, std::vec::IntoIter<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter().zip(self.labels)
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

pub fn parse_address(address: &str) -> AddressParserResponse {
    let address = CString::new(address).unwrap();
    let mut response = AddressParserResponse {
        components: Vec::new(),
        labels: Vec::new(),
    };

    unsafe {
        let options = ffi::libpostal_get_address_parser_default_options();
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
