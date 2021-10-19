/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Listing : A real estate listing.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Listing {
    /// String representing the object’s type. Objects of the same type share the same schema.
    #[serde(rename = "object")]
    pub object: String,
    /// ID of the listing. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "address")]
    pub address: Box<crate::models::Address>,
    /// The identifier for a listing on its local MLS. 
    #[serde(rename = "mls_number", skip_serializing_if = "Option::is_none")]
    pub mls_number: Option<String>,
    /// General type of the listing, primarily categorizing its use case. Examples include residential and commercial. 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Further specifies the listing type. Examples include family residence and condominium.
    #[serde(rename = "sub_type", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<SubType>,
    /// Local, regional, or otherwise custom status for the listing used by the parties involved in the listing transaction. While variable, these statuses are typically mapped to the listing's standard status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The status of the listing as it reflects the state of the contract between the listing agent and seller or an agreement with a buyer, including Active, Active Under Contract, Canceled, Closed, Expired, Pending, and Withdrawn.
    #[serde(rename = "standard_status", skip_serializing_if = "Option::is_none")]
    pub standard_status: Option<StandardStatus>,
    /// Description of the selling points of the building and/or land for sale. 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lot", skip_serializing_if = "Option::is_none")]
    pub lot: Option<Box<crate::models::ListingLot>>,
    #[serde(rename = "building", skip_serializing_if = "Option::is_none")]
    pub building: Option<Box<crate::models::ListingBuilding>>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::models::ListingPrice>>,
    #[serde(rename = "list_agent", skip_serializing_if = "Option::is_none")]
    pub list_agent: Option<Box<crate::models::Group>>,
    #[serde(rename = "co_list_agent", skip_serializing_if = "Option::is_none")]
    pub co_list_agent: Option<Box<crate::models::Group>>,
    /// images
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::Image>>,
    /// videos
    #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<crate::models::Video>>,
    /// floor_plans
    #[serde(rename = "floor_plans", skip_serializing_if = "Option::is_none")]
    pub floor_plans: Option<Vec<crate::models::FloorPlan>>,
    /// interactive_content
    #[serde(rename = "interactive_content", skip_serializing_if = "Option::is_none")]
    pub interactive_content: Option<Vec<crate::models::InteractiveContent>>,
    #[serde(rename = "property_website", skip_serializing_if = "Option::is_none")]
    pub property_website: Option<Box<crate::models::PropertyWebsite>>,
    /// orders
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<crate::models::Order>>,
    /// Are downloads enabled for this listing?
    #[serde(rename = "downloads_enabled")]
    pub downloads_enabled: bool,
}

impl Listing {
    /// A real estate listing.
    pub fn new(object: String, id: String, address: crate::models::Address, downloads_enabled: bool) -> Listing {
        Listing {
            object,
            id,
            address: Box::new(address),
            mls_number: None,
            _type: None,
            sub_type: None,
            status: None,
            standard_status: None,
            description: None,
            lot: None,
            building: None,
            price: None,
            list_agent: None,
            co_list_agent: None,
            images: None,
            videos: None,
            floor_plans: None,
            interactive_content: None,
            property_website: None,
            orders: None,
            downloads_enabled,
        }
    }
}

/// General type of the listing, primarily categorizing its use case. Examples include residential and commercial. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BUSINESS_OPPORTUNITY")]
    BUSINESSOPPORTUNITY,
    #[serde(rename = "COMMERCIAL_LEASE")]
    COMMERCIALLEASE,
    #[serde(rename = "COMMERCIAL_SALE")]
    COMMERCIALSALE,
    #[serde(rename = "FARM")]
    FARM,
    #[serde(rename = "LAND")]
    LAND,
    #[serde(rename = "MANUFACTURED_IN_PARK")]
    MANUFACTUREDINPARK,
    #[serde(rename = "RESIDENTIAL")]
    RESIDENTIAL,
    #[serde(rename = "RESIDENTIAL_INCOME")]
    RESIDENTIALINCOME,
    #[serde(rename = "RESIDENTIAL_LEASE")]
    RESIDENTIALLEASE,
}
/// Further specifies the listing type. Examples include family residence and condominium.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubType {
    #[serde(rename = "APARTMENT")]
    APARTMENT,
    #[serde(rename = "CONDOMINIUM")]
    CONDOMINIUM,
    #[serde(rename = "DUPLEX")]
    DUPLEX,
    #[serde(rename = "FARM")]
    FARM,
    #[serde(rename = "SINGLE_FAMILY_RESIDENCE")]
    SINGLEFAMILYRESIDENCE,
    #[serde(rename = "TIMESHARE")]
    TIMESHARE,
    #[serde(rename = "TOWNHOUSE")]
    TOWNHOUSE,
    #[serde(rename = "OFFICE")]
    OFFICE,
}
/// Local, regional, or otherwise custom status for the listing used by the parties involved in the listing transaction. While variable, these statuses are typically mapped to the listing's standard status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DRAFT")]
    DRAFT,
    #[serde(rename = "COMING_SOON")]
    COMINGSOON,
    #[serde(rename = "FOR_SALE")]
    FORSALE,
    #[serde(rename = "FOR_LEASE")]
    FORLEASE,
    #[serde(rename = "PENDING_SALE")]
    PENDINGSALE,
    #[serde(rename = "PENDING_LEASE")]
    PENDINGLEASE,
    #[serde(rename = "SOLD")]
    SOLD,
    #[serde(rename = "LEASED")]
    LEASED,
    #[serde(rename = "OFF_MARKET")]
    OFFMARKET,
}
/// The status of the listing as it reflects the state of the contract between the listing agent and seller or an agreement with a buyer, including Active, Active Under Contract, Canceled, Closed, Expired, Pending, and Withdrawn.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StandardStatus {
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "ACTIVE_UNDER_CONTRACT")]
    ACTIVEUNDERCONTRACT,
    #[serde(rename = "CANCELED")]
    CANCELED,
    #[serde(rename = "CLOSED")]
    CLOSED,
    #[serde(rename = "COMING_SOON")]
    COMINGSOON,
    #[serde(rename = "DELETE")]
    DELETE,
    #[serde(rename = "EXPIRED")]
    EXPIRED,
    #[serde(rename = "HOLD")]
    HOLD,
    #[serde(rename = "INCOMPLETE")]
    INCOMPLETE,
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "WITHDRAWN")]
    WITHDRAWN,
}

