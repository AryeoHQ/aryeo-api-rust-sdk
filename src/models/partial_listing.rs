/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PartialListing : A real-estate property.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialListing {
    /// ID of the listing.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "address")]
    pub address: Box<crate::models::PartialAddress>,
    /// Has the listing been delivered?
    #[serde(rename = "delivery_status")]
    pub delivery_status: DeliveryStatus,
    /// Thumbnail URL for the listing.
    #[serde(rename = "thumbnail_url", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// The price of the property in dollars.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// URL for branded property website.
    #[serde(rename = "branded_url", skip_serializing_if = "Option::is_none")]
    pub branded_url: Option<String>,
    /// Total number of square feet.
    #[serde(rename = "square_feet", skip_serializing_if = "Option::is_none")]
    pub square_feet: Option<f32>,
    /// Number of bedrooms.
    #[serde(rename = "bedrooms", skip_serializing_if = "Option::is_none")]
    pub bedrooms: Option<i32>,
    /// Number of bathrooms.
    #[serde(rename = "bathrooms", skip_serializing_if = "Option::is_none")]
    pub bathrooms: Option<f32>,
    /// Are downloads enabled for this listing?
    #[serde(rename = "downloads_enabled")]
    pub downloads_enabled: bool,
    /// Sales status for the listing.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "property_details", skip_serializing_if = "Option::is_none")]
    pub property_details: Option<Box<crate::models::PropertyDetails>>,
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<Box<crate::models::PartialGroup>>,
    #[serde(rename = "co_agent", skip_serializing_if = "Option::is_none")]
    pub co_agent: Option<Box<crate::models::PartialGroup>>,
}

impl PartialListing {
    /// A real-estate property.
    pub fn new(id: String, address: crate::models::PartialAddress, delivery_status: DeliveryStatus, downloads_enabled: bool) -> PartialListing {
        PartialListing {
            id,
            address: Box::new(address),
            delivery_status,
            thumbnail_url: None,
            price: None,
            branded_url: None,
            square_feet: None,
            bedrooms: None,
            bathrooms: None,
            downloads_enabled,
            status: None,
            property_details: None,
            agent: None,
            co_agent: None,
        }
    }
}

/// Has the listing been delivered?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeliveryStatus {
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "undelivered")]
    Undelivered,
}
/// Sales status for the listing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Off Market")]
    OffMarket,
    #[serde(rename = "Pending Lease")]
    PendingLease,
    #[serde(rename = "Vacation Rental")]
    VacationRental,
    #[serde(rename = "Leased")]
    Leased,
    #[serde(rename = "For Sale")]
    ForSale,
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Coming Soon")]
    ComingSoon,
    #[serde(rename = "Soldm")]
    Soldm,
    #[serde(rename = "NULL")]
    NULL,
    #[serde(rename = "For Lease")]
    ForLease,
    #[serde(rename = "Pending Sale")]
    PendingSale,
    #[serde(rename = "Price Reduction")]
    PriceReduction,
    #[serde(rename = "Sold")]
    Sold,
}

