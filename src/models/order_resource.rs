/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// OrderResource : An order.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderResource {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::Order>>,
}

impl OrderResource {
    /// An order.
    pub fn new() -> OrderResource {
        OrderResource {
            data: None,
        }
    }
}


