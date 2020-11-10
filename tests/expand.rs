extern crate rustpostal;

use rustpostal::expand;

const TEST_CASES: &[(&str, &str, &str)] = &[
    ("123 Main St. #2f", "123 main street number 2f", "en"),
    ("120 E 96th St", "120 east 96 street", "en"),
    ("120 E Ninety-sixth St", "120 east 96 street", "en"),
    (
        "4998 Vanderbilt Dr, Columbus, OH 43213",
        "4998 vanderbilt drive columbus ohio 43213",
        "en",
    ),
    (
        "Nineteen oh one W El Segundo Blvd",
        "1901 west el segundo boulevard",
        "en",
    ),
    ("S St. NW", "s street northwest", "en"),
    (
        "Quatre vingt douze Ave des Champs-Élysées",
        "92 avenue des champs-elysees",
        "fr",
    ),
    (
        "Quatre vingt douze Ave des Champs-Élysées",
        "92 avenue des champs elysees",
        "fr",
    ),
    (
        "Quatre vingt douze Ave des Champs-Élysées",
        "92 avenue des champselysees",
        "fr",
    ),
    ("Marktstrasse", "markt strasse", "de"),
    ("Hoofdstraat", "hoofdstraat", "nl"),
    ("มงแตร", "มงแตร", "th"),
];

fn expansion_contains_phrase(address: &str, phrase: &str) -> bool {
    let expansion = expand::expand_address(address);
    for expanded in expansion {
        if expanded == phrase {
            return true;
        }
    }
    false
}

fn expansion_contains_phrase_with_options(address: &str, phrase: &str, lang: &str) -> bool {
    let expansion = expand::expand_address_with_options(address, Some(vec![lang]));
    for expanded in expansion {
        if expanded == phrase {
            return true;
        }
    }
    false
}

#[test]
fn expand() {
    unsafe { rustpostal::setup(Some("expand")) };
    for (address, phrase, _) in TEST_CASES {
        assert!(expansion_contains_phrase(address, phrase));
    }
    unsafe { rustpostal::teardown(Some("expand")) };
}

#[test]
fn expand_with_options() {
    unsafe { rustpostal::setup(Some("expand")) };
    for (address, phrase, lang) in TEST_CASES {
        assert!(expansion_contains_phrase_with_options(
            address, phrase, lang
        ));
    }
    unsafe { rustpostal::teardown(Some("expand")) };
}
