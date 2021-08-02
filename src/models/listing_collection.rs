/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ListingCollection : A collection of listings.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListingCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Listing>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl ListingCollection {
    /// A collection of listings.
    pub fn new(status: String) -> ListingCollection {
        ListingCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


