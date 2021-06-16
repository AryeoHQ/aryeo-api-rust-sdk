/*
 * Aryeo
 *
 * # Introduction The Aryeo API gives access to the Aryeo platform. You can use your favorite HTTP/REST library for interfacing with the Aryeo API, or you can use one of our SDKs. Our SDKs are procedurally generated and a great starting point for experimental testing. If there is an additional language or framework that you want to see supported, then please reach and out and make a contribution!  <p align=\"center\"> <a href=\"https://github.com/AryeoHQ/aryeo-api-dart-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/dart.svg\" alt=\"Dart\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> <a href=\"https://github.com/AryeoHQ/aryeo-api-go-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/go.svg\" alt=\"Go\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> <a href=\"https://github.com/AryeoHQ/aryeo-api-js-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/js.svg\" alt=\"Node JS\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> <a href=\"https://github.com/AryeoHQ/aryeo-api-php-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/php.svg\" alt=\"PHP\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> <a href=\"https://github.com/AryeoHQ/aryeo-api-ruby-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/ruby.svg\" alt=\"Ruby\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> <a href=\"https://github.com/AryeoHQ/aryeo-api-rust-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/rust.svg\" alt=\"Rust\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> <a href=\"https://github.com/AryeoHQ/aryeo-api-swift-sdk\"><img src=\"https://raw.githubusercontent.com/AryeoHQ/aryeo-api-docs/master/public/images/swift.svg\" alt=\"Swift\" width=\"44\" style=\"padding:10px;border: 1px solid #d3d3d3;border-radius: 5px;margin:4px;\"/></a> </p>  **Note**: Some SDKs may require you to manually add the `Accept` header to all Aryeo API requests. If so, use the value `application/json`.  # Authentication To start using the Aryeo API, you will need to generate an API key from your group's developer settings. You can then authenticate to the Aryeo API by providing your key in the appropriate request header. Requests made without an API key will result in a `401 Unauthorized` error. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: jarrod@aryeo.com
 * Generated by: https://openapi-generator.tech
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
