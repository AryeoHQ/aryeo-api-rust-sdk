/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// OrderPostPayload : Payload for creating an order.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderPostPayload {
    /// The fulfillment status of the order. Defaults to \"UNFULFILLED\".
    #[serde(rename = "fulfillment_status", skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<FulfillmentStatus>,
    /// The payment status of the order. Defaults to \"UNPAID\". 
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<PaymentStatus>,
    /// Google Places ID of the address to attach to the order.
    #[serde(rename = "place_id", skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
}

impl OrderPostPayload {
    /// Payload for creating an order.
    pub fn new() -> OrderPostPayload {
        OrderPostPayload {
            fulfillment_status: None,
            payment_status: None,
            place_id: None,
        }
    }
}

/// The fulfillment status of the order. Defaults to \"UNFULFILLED\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FulfillmentStatus {
    #[serde(rename = "FULFILLED")]
    FULFILLED,
    #[serde(rename = "UNFULFILLED")]
    UNFULFILLED,
}
/// The payment status of the order. Defaults to \"UNPAID\". 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "PAID")]
    PAID,
    #[serde(rename = "UNPAID")]
    UNPAID,
}

