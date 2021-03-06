/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// GroupCollection : A collection of groups.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Group>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl GroupCollection {
    /// A collection of groups.
    pub fn new(status: String) -> GroupCollection {
        GroupCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


