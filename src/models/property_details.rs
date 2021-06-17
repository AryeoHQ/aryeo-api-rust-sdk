/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PropertyDetails : Details about a real-estate listing.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyDetails {
    /// The price of the property in dollars.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// A number assigned to a real-estate listing for the purposes of information sharing.
    #[serde(rename = "mls_number", skip_serializing_if = "Option::is_none")]
    pub mls_number: Option<String>,
    /// Number of bedrooms.
    #[serde(rename = "bedrooms", skip_serializing_if = "Option::is_none")]
    pub bedrooms: Option<i32>,
    /// Number of bathrooms. Represented as a number in order to support half-baths.
    #[serde(rename = "bathrooms", skip_serializing_if = "Option::is_none")]
    pub bathrooms: Option<f32>,
    /// Total number of square feet.
    #[serde(rename = "square_feet", skip_serializing_if = "Option::is_none")]
    pub square_feet: Option<f32>,
    /// Total acres.
    #[serde(rename = "lot_acres", skip_serializing_if = "Option::is_none")]
    pub lot_acres: Option<f32>,
    /// Number of parking spaces.
    #[serde(rename = "parking_spots", skip_serializing_if = "Option::is_none")]
    pub parking_spots: Option<i32>,
    /// Year the property was built.
    #[serde(rename = "year_built", skip_serializing_if = "Option::is_none")]
    pub year_built: Option<i32>,
    /// Type of property.
    #[serde(rename = "property_type", skip_serializing_if = "Option::is_none")]
    pub property_type: Option<PropertyType>,
    /// Property description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PropertyDetails {
    /// Details about a real-estate listing.
    pub fn new() -> PropertyDetails {
        PropertyDetails {
            price: None,
            mls_number: None,
            bedrooms: None,
            bathrooms: None,
            square_feet: None,
            lot_acres: None,
            parking_spots: None,
            year_built: None,
            property_type: None,
            description: None,
        }
    }
}

/// Type of property.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    #[serde(rename = "Single Family")]
    SingleFamily,
    #[serde(rename = "Single Family Attached")]
    SingleFamilyAttached,
    #[serde(rename = "Single Family Detached")]
    SingleFamilyDetached,
    #[serde(rename = "Colonial")]
    Colonial,
    #[serde(rename = "Condo")]
    Condo,
    #[serde(rename = "Townhome")]
    Townhome,
    #[serde(rename = "Twinhome")]
    Twinhome,
    #[serde(rename = "Duplex")]
    Duplex,
    #[serde(rename = "Lot")]
    Lot,
    #[serde(rename = "Land")]
    Land,
    #[serde(rename = "Manufactured Home")]
    ManufacturedHome,
    #[serde(rename = "Semi-Detached")]
    SemiDetached,
    #[serde(rename = "Rental")]
    Rental,
    #[serde(rename = "Row House")]
    RowHouse,
    #[serde(rename = "Horse Farm")]
    HorseFarm,
    #[serde(rename = "Other")]
    Other,
}

