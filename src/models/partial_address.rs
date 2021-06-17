/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PartialAddress : A structure containing a street address and additional metadata about a location.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialAddress {
    /// ID of address.
    #[serde(rename = "id")]
    pub id: i32,
    /// The full address string containing address_1 and address_2.
    #[serde(rename = "full_address", skip_serializing_if = "Option::is_none")]
    pub full_address: Option<String>,
    /// A formatted address string containing the street.
    #[serde(rename = "formatted_address_1")]
    pub formatted_address_1: String,
    /// A formatted address string containing the city, state, and zipcode.
    #[serde(rename = "formatted_address_2")]
    pub formatted_address_2: String,
    /// Latitude of the address.
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// Longitude of the address.
    #[serde(rename = "longitude")]
    pub longitude: f64,
    /// ID of place.
    #[serde(rename = "place_id", skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
    /// Address line 1
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    /// Address line 2
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// City
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Postal Code
    #[serde(rename = "postal_code", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl PartialAddress {
    /// A structure containing a street address and additional metadata about a location.
    pub fn new(id: i32, formatted_address_1: String, formatted_address_2: String, latitude: f64, longitude: f64) -> PartialAddress {
        PartialAddress {
            id,
            full_address: None,
            formatted_address_1,
            formatted_address_2,
            latitude,
            longitude,
            place_id: None,
            address_line_1: None,
            address_line_2: None,
            city: None,
            state: None,
            postal_code: None,
        }
    }
}


