/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// OrderPostPayload : Payload for creating an order.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderPostPayload {
    /// The fulfillment status of the order.
    #[serde(rename = "fulfillment_status", skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<FulfillmentStatus>,
    /// The payment status of the order.
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<PaymentStatus>,
    /// product_items
    #[serde(rename = "product_items", skip_serializing_if = "Option::is_none")]
    pub product_items: Option<Vec<crate::models::ProductItem>>,
}

impl OrderPostPayload {
    /// Payload for creating an order.
    pub fn new() -> OrderPostPayload {
        OrderPostPayload {
            fulfillment_status: None,
            payment_status: None,
            product_items: None,
        }
    }
}

/// The fulfillment status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FulfillmentStatus {
    #[serde(rename = "fulfilled")]
    Fulfilled,
    #[serde(rename = "unfulfilled")]
    Unfulfilled,
}
/// The payment status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "unpaid")]
    Unpaid,
}

