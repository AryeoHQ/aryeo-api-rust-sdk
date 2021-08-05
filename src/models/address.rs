/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Address : A street address and additional metadata about a location.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// ID of the address. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The geographic latitude of some reference point of the location, specified in degrees and decimal parts. Positive numbers must not include the plus symbol.
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// The geographic longitude of some reference point of the location, specified in degrees and decimal parts. Positive numbers must not include the plus symbol.
    #[serde(rename = "longitude")]
    pub longitude: f32,
    /// The street number portion of a location's address. In some areas, the street number may contain non-numeric characters. This field can also contain extensions and modifiers to the street number, such as \"1/2\" or \"-B\".
    #[serde(rename = "street_number", skip_serializing_if = "Option::is_none")]
    pub street_number: Option<String>,
    /// The street name portion of a location's address.
    #[serde(rename = "street_name", skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,
    /// The number or portion of a larger building or complex. Examples are: \"APT G\", \"55\", etc.
    #[serde(rename = "unit_number", skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<String>,
    /// The postal code portion of a location's address.
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The city of a location's address.
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// A sub-section or area of a defined city. Examples would be SoHo in New York, NY, Ironbound in Newark, NJ or Inside the Beltway.
    #[serde(rename = "city_region", skip_serializing_if = "Option::is_none")]
    pub city_region: Option<String>,
    /// The County, Parish or other regional authority of the location.
    #[serde(rename = "county_or_parish", skip_serializing_if = "Option::is_none")]
    pub county_or_parish: Option<String>,
    /// The ISO 3166-2 subdivision code for the state or province of the location. For example, “MA” for Massachusetts, United States.
    #[serde(rename = "state_or_province", skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    /// A sub-section or area of a defined state or province. Examples would be the Keys in FL or Hudson Valley in NY.
    #[serde(rename = "state_or_province_region", skip_serializing_if = "Option::is_none")]
    pub state_or_province_region: Option<String>,
    /// The ISO 3166-1 country code for this for the country of the location.
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// A sub-section or area of a defined country. Examples would be Napa Valley in the US, or the Amalfi Coast in Italy.
    #[serde(rename = "country_region", skip_serializing_if = "Option::is_none")]
    pub country_region: Option<String>,
    /// Unparsed text representation of the address. 
    #[serde(rename = "unparsed_address", skip_serializing_if = "Option::is_none")]
    pub unparsed_address: Option<String>,
    /// Unparsed text representation of the first part of the address, typically including the street number, street name, and unit number if applicable.  
    #[serde(rename = "unparsed_address_part_one", skip_serializing_if = "Option::is_none")]
    pub unparsed_address_part_one: Option<String>,
    /// Unparsed text representation of the second part of the address, typically including the city, state or province, and postal code.  
    #[serde(rename = "unparsed_address_part_two", skip_serializing_if = "Option::is_none")]
    pub unparsed_address_part_two: Option<String>,
}

impl Address {
    /// A street address and additional metadata about a location.
    pub fn new(id: String, latitude: f32, longitude: f32) -> Address {
        Address {
            id,
            latitude,
            longitude,
            street_number: None,
            street_name: None,
            unit_number: None,
            postal_code: None,
            city: None,
            city_region: None,
            county_or_parish: None,
            state_or_province: None,
            state_or_province_region: None,
            country: None,
            country_region: None,
            unparsed_address: None,
            unparsed_address_part_one: None,
            unparsed_address_part_two: None,
        }
    }
}


