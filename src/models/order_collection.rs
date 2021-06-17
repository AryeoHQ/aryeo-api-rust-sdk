/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// OrderCollection : A collection of orders.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderCollection {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Order>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl OrderCollection {
    /// A collection of orders.
    pub fn new() -> OrderCollection {
        OrderCollection {
            data: None,
            meta: None,
            links: None,
        }
    }
}


