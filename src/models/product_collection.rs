/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ProductCollection : A collection of products.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Product>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl ProductCollection {
    /// A collection of products.
    pub fn new(status: String) -> ProductCollection {
        ProductCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


