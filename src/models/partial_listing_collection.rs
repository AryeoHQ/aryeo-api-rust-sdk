/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PartialListingCollection : A collection of partial listings.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialListingCollection {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::PartialListing>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl PartialListingCollection {
    /// A collection of partial listings.
    pub fn new() -> PartialListingCollection {
        PartialListingCollection {
            data: None,
            meta: None,
            links: None,
        }
    }
}


