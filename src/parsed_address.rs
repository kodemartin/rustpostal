use crate::address::AddressParserResponse;

#[derive(Clone, Default, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ParsedAddress {
    pub house: Option<String>,
    pub house_number: Option<String>,
    pub po_box: Option<String>,
    pub building: Option<String>,
    pub entrance: Option<String>,
    pub staircase: Option<String>,
    pub level: Option<String>,
    pub unit: Option<String>,
    pub road: Option<String>,
    pub metro_station: Option<String>,
    pub suburb: Option<String>,
    pub city_district: Option<String>,
    pub city: Option<String>,
    pub state_district : Option<String>,
    pub island: Option<String>,
    pub state : Option<String>,
    pub postal_code : Option<String>,
    pub country_region : Option<String>,
    pub country : Option<String>,
    pub world_region: Option<String>,
    pub website: Option<String>,
    pub telephone: Option<String>,
}

impl From<AddressParserResponse> for ParsedAddress {
    fn from(response: AddressParserResponse) -> Self {
        let mut parsed_address = ParsedAddress::default();

        // this seems like a lot of cloning...
        for (label, token) in &response {
            match label.as_str() {
                "house" => parsed_address.house = Some((*token).clone()),
                "house_number" => parsed_address.house_number = Some((*token).clone()),
                "po_box" => parsed_address.po_box = Some((*token).clone()),
                "building" => parsed_address.building = Some((*token).clone()),
                "entrance" => parsed_address.entrance = Some((*token).clone()),
                "staircase" => parsed_address.staircase = Some((*token).clone()),
                "level" => parsed_address.level = Some((*token).clone()),
                "unit" => parsed_address.unit = Some((*token).clone()),
                "road" => parsed_address.road = Some((*token).clone()),
                "metro_station" => parsed_address.metro_station = Some((*token).clone()),
                "suburb" => parsed_address.suburb = Some((*token).clone()),
                "city_district" => parsed_address.city_district = Some((*token).clone()),
                "city" => parsed_address.city = Some((*token).clone()),
                "state_district" => parsed_address.state_district = Some((*token).clone()),
                "island" => parsed_address.island = Some((*token).clone()),
                "state" => parsed_address.state = Some((*token).clone()),
                "postcode" => parsed_address.postal_code = Some((*token).clone()),
                "country_region" => parsed_address.country_region = Some((*token).clone()),
                "country" => parsed_address.country = Some((*token).clone()),
                "world_region" => parsed_address.world_region = Some((*token).clone()),
                "website" => parsed_address.website = Some((*token).clone()),
                "phone" => parsed_address.telephone = Some((*token).clone()),
                _ => (),
            }
        }
        parsed_address
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_address_default() {
        let parsed_address = ParsedAddress::default();
        assert_eq!(parsed_address.house, None);
        assert_eq!(parsed_address.house_number, None);
        assert_eq!(parsed_address.po_box, None);
        assert_eq!(parsed_address.building, None);
        assert_eq!(parsed_address.entrance, None);
        assert_eq!(parsed_address.staircase, None);
        assert_eq!(parsed_address.level, None);
        assert_eq!(parsed_address.unit, None);
        assert_eq!(parsed_address.road, None);
        assert_eq!(parsed_address.metro_station, None);
        assert_eq!(parsed_address.suburb, None);
        assert_eq!(parsed_address.city_district, None);
        assert_eq!(parsed_address.city, None);
        assert_eq!(parsed_address.state_district , None);
        assert_eq!(parsed_address.island, None);
        assert_eq!(parsed_address.state , None);
        assert_eq!(parsed_address.postal_code , None);
        assert_eq!(parsed_address.country_region , None);
        assert_eq!(parsed_address.country , None);
        assert_eq!(parsed_address.world_region, None);
        assert_eq!(parsed_address.website, None);
        assert_eq!(parsed_address.telephone, None);
    }
}
