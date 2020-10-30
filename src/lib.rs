use libc;

pub mod postal {
    use super::*;

    #[link(name = "postal")]
    extern "C" {
        pub fn libpostal_get_address_parser_default_options() -> libpostal_address_parser_options;
        pub fn libpostal_setup() -> bool;
        pub fn libpostal_setup_parser() -> bool;
        pub fn libpostal_teardown() -> bool;
        pub fn libpostal_teardown_parser() -> bool;

        pub fn libpostal_parse_address(
            address: *const libc::c_char,
            options: libpostal_address_parser_options,
        ) -> *const libpostal_address_parser_response;

        pub fn libpostal_address_parser_response_destroy(response: *const libpostal_address_parser_response);
    }

    #[repr(C)]
    pub struct libpostal_address_parser_options {
        language: *const libc::c_char,
        country: *const libc::c_char,
    }

    #[repr(C)]
    pub struct libpostal_address_parser_response {
        pub num_components: usize,
        pub components: *mut *const libc::c_char,
        pub labels: *mut *const libc::c_char,
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
