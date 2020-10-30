use rustpostal::postal::*;
use std::process;
use std::ffi::CStr;

fn main() {
    unsafe{
        if !libpostal_setup() || !libpostal_setup_parser() {
            process::exit(1);
        }

        let options = libpostal_get_address_parser_default_options();
        let address = b"781 Franklin Ave Crown Heights Brooklyn NYC NY 11216 USA\0";
        let response = libpostal_parse_address(address.as_ptr() as *const libc::c_char, options);

        if let Some(parsed) = response.as_ref() {
            for i in 0..parsed.num_components {
                let component = CStr::from_ptr(*parsed.components.add(i));
                let label = CStr::from_ptr(*parsed.labels.add(i));
                println!("{}: {}", component.to_str().unwrap(),
                         label.to_str().unwrap());
            }
        };

        libpostal_address_parser_response_destroy(response);

        libpostal_teardown();
        libpostal_teardown_parser();
    }
}
