use rustpostal::*;

fn main() {
    setup();

    let address = "781 Franklin Ave Crown Heights Brooklyn NYC NY 11216 USA";

    let response = parse_address(address);
    for (component, label) in response {
        println!("{}: {}", component, label);
    }
    teardown();
}
