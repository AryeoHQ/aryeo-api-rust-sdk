/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// OrderItem : A product associated with an order.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItem {
    /// ID of the item. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The title of the item.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The description of the item.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00) representing the cost of a single instance of this item. This is multiplied by the quantity to calculate what was or will be charged.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// A positive integer representing the number of instances of this item that was or will be charged.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl OrderItem {
    /// A product associated with an order.
    pub fn new(id: String) -> OrderItem {
        OrderItem {
            id,
            title: None,
            description: None,
            amount: None,
            quantity: None,
        }
    }
}


