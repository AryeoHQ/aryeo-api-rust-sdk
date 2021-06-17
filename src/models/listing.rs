/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Listing : A real-estate property.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
    /// ID of the listing.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "address")]
    pub address: Box<crate::models::PartialAddress>,
    /// Has this listing been delivered?
    #[serde(rename = "delivery_status")]
    pub delivery_status: DeliveryStatus,
    /// Thumbnail URL for the listing.
    #[serde(rename = "thumbnail_url", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "agent", skip_serializing_if = "Option::is_none")]
    pub agent: Option<Box<crate::models::Group>>,
    #[serde(rename = "co_agent", skip_serializing_if = "Option::is_none")]
    pub co_agent: Option<Box<crate::models::Group>>,
    /// images
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::Image>>,
    /// videos
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<crate::models::Video>>,
    /// floor_plans
    #[serde(rename = "floor_plans", skip_serializing_if = "Option::is_none")]
    pub floor_plans: Option<Vec<crate::models::FloorPlan>>,
    #[serde(rename = "property_websites", skip_serializing_if = "Option::is_none")]
    pub property_websites: Option<Box<crate::models::PropertyWebsites>>,
    /// interactive_content
    #[serde(rename = "interactive_content", skip_serializing_if = "Option::is_none")]
    pub interactive_content: Option<Vec<crate::models::InteractiveContent>>,
    #[serde(rename = "property_details", skip_serializing_if = "Option::is_none")]
    pub property_details: Option<Box<crate::models::PropertyDetails>>,
    /// Are downloads enabled for this listing?
    #[serde(rename = "downloads_enabled")]
    pub downloads_enabled: bool,
    /// orders
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<crate::models::Order>>,
}

impl Listing {
    /// A real-estate property.
    pub fn new(id: String, address: crate::models::PartialAddress, delivery_status: DeliveryStatus, downloads_enabled: bool) -> Listing {
        Listing {
            id,
            address: Box::new(address),
            delivery_status,
            thumbnail_url: None,
            agent: None,
            co_agent: None,
            images: None,
            videos: None,
            floor_plans: None,
            property_websites: None,
            interactive_content: None,
            property_details: None,
            downloads_enabled,
            orders: None,
        }
    }
}

/// Has this listing been delivered?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeliveryStatus {
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "undelivered")]
    Undelivered,
}

