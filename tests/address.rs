extern crate rustpostal;

fn collect_actual(address: &str) -> Vec<(String, String)> {
    rustpostal::address::parse_address(address, None, None)
        .into_iter()
        .collect()
}

fn collect_expected(expected: Vec<(&str, &str)>) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(c, l)| (String::from(*c), String::from(*l)))
        .collect()
}

fn assert_actual_eq_expected(address: &str, expected: Vec<(&str, &str)>) {
    let actual = collect_actual(address);
    let expected = collect_expected(expected);
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

#[test]
fn parse() {
    unsafe { rustpostal::setup() }
    us_parse();
    gb_parse();
    es_parse();
    unsafe { rustpostal::teardown() }
}
