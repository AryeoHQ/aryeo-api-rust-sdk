/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ListingResource : A listing.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListingResource {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::Listing>>,
}

impl ListingResource {
    /// A listing.
    pub fn new() -> ListingResource {
        ListingResource {
            data: None,
        }
    }
}


