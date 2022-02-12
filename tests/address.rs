extern crate rustpostal;
use rustpostal::address::ParsedAddress;
use rustpostal::error::RuntimeError;
use rustpostal::LibModules;

fn assert_actual_eq_expected(address: &str, expected: Vec<(&str, &str)>) {
    let response = rustpostal::address::parse_address(address, None, None).unwrap();
    let actual: Vec<(&str, &str)> = response
        .into_iter()
        .map(|(l, t)| (l.as_str(), t.as_str()))
        .collect();
    assert_eq!(actual, expected);
}

fn us_parse() {
    let address = "Black Alliance for Just Immigration 660 Nostrand Ave, Brooklyn, N.Y., 11216";
    let expected = vec![
        ("house", "black alliance for just immigration"),
        ("house_number", "660"),
        ("road", "nostrand ave"),
        ("city_district", "brooklyn"),
        ("state", "n.y."),
        ("postcode", "11216"),
    ];
    assert_actual_eq_expected(address, expected);
}

fn us_parse_to_struct() {
    let address = "Black Alliance for Just Immigration 660 Nostrand Ave, Brooklyn, N.Y., 11216";
    let response = rustpostal::address::parse_address(address, None, None).unwrap();
    let actual = ParsedAddress::from(response);
    assert_eq!(
        actual.house(),
        Some("black alliance for just immigration".to_string())
    );
    assert_eq!(actual.house_number(), Some("660".to_string()));
    assert_eq!(actual.road(), Some("nostrand ave".to_string()));
    assert_eq!(actual.city_district(), Some("brooklyn".to_string()));
    assert_eq!(actual.state(), Some("n.y.".to_string()));
    assert_eq!(actual.postcode(), Some("11216".to_string()));
}

fn gb_parse() {
    let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
    let expected = vec![
        ("house", "st johns centre"),
        ("road", "rope walk"),
        ("city", "bedford"),
        ("state_district", "bedfordshire"),
        ("postcode", "mk42 0xe"),
        ("country", "united kingdom"),
    ];
    assert_actual_eq_expected(address, expected);
}

fn gb_parse_to_struct() {
    let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
    let response = rustpostal::address::parse_address(address, None, None).unwrap();
    let actual = ParsedAddress::from(response);
    assert_eq!(actual.house(), Some("st johns centre".to_string()));
    assert_eq!(actual.road(), Some("rope walk".to_string()));
    assert_eq!(actual.city(), Some("bedford".to_string()));
    assert_eq!(actual.state_district(), Some("bedfordshire".to_string()));
    assert_eq!(actual.postcode(), Some("mk42 0xe".to_string()));
    assert_eq!(actual.country(), Some("united kingdom".to_string()));
}

fn es_parse() {
    let address = "Museo del Prado C. de Ruiz de Alarcón,
                   23 28014 Madrid, España";
    let expected = vec![
        ("house", "museo del prado"),
        ("road", "c. de ruiz de alarcón"),
        ("house_number", "23"),
        ("postcode", "28014"),
        ("city", "madrid"),
        ("country", "españa"),
    ];
    assert_actual_eq_expected(address, expected);
}

fn es_parse_to_struct() {
    let address = "Museo del Prado C. de Ruiz de Alarcón,
                   23 28014 Madrid, España";
    let response = rustpostal::address::parse_address(address, None, None).unwrap();
    let actual = ParsedAddress::from(response);
    assert_eq!(actual.house(), Some("museo del prado".to_string()));
    assert_eq!(actual.road(), Some("c. de ruiz de alarcón".to_string()));
    assert_eq!(actual.house_number(), Some("23".to_string()));
    assert_eq!(actual.postcode(), Some("28014".to_string()));
    assert_eq!(actual.city(), Some("madrid".to_string()));
    assert_eq!(actual.country(), Some("españa".to_string()));
}

#[test]
fn parse() -> Result<(), RuntimeError> {
    let postal_module = LibModules::Address;
    postal_module.setup()?;
    us_parse();
    gb_parse();
    es_parse();
    Ok(())
}

#[test]
fn parse_address_to_parsed_address_struct() -> Result<(), RuntimeError> {
    let postal_module = LibModules::Address;
    postal_module.setup()?;
    us_parse_to_struct();
    gb_parse_to_struct();
    es_parse_to_struct();
    Ok(())
}
