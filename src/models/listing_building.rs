/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ListingBuilding : Structural data of the primary building on a listing. Includes everything from square footage of certain spaces to construction dates. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListingBuilding {
    /// Total number of bedrooms.
    #[serde(rename = "bedrooms", skip_serializing_if = "Option::is_none")]
    pub bedrooms: Option<i32>,
    /// Sum of the number of bathrooms. Represented as a number in order to support half-baths.
    #[serde(rename = "bathrooms", skip_serializing_if = "Option::is_none")]
    pub bathrooms: Option<f32>,
    /// Total number of square feet.
    #[serde(rename = "square_feet", skip_serializing_if = "Option::is_none")]
    pub square_feet: Option<f32>,
    /// Year the property was built.
    #[serde(rename = "year_built", skip_serializing_if = "Option::is_none")]
    pub year_built: Option<i32>,
}

impl ListingBuilding {
    /// Structural data of the primary building on a listing. Includes everything from square footage of certain spaces to construction dates. 
    pub fn new() -> ListingBuilding {
        ListingBuilding {
            bedrooms: None,
            bathrooms: None,
            square_feet: None,
            year_built: None,
        }
    }
}


