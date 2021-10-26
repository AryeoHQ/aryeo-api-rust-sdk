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
    /// Internal notes that will be attached to the order. Viewable only by the team.
    #[serde(rename = "internal_notes", skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// The payment status of the order. Defaults to \"UNPAID\". 
    #[serde(rename = "payment_status", skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<PaymentStatus>,
    /// ID of the address to associate with the order. UUID Version 4.
    #[serde(rename = "address_id", skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// ID of the customer to associate with the order. UUID Version 4.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Indicates if the customer and creator notifications should be sent when creating the order. Requires an address and customer to be set in order for the notifications to be sent.
    #[serde(rename = "notify", skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
}

impl OrderPostPayload {
    /// Payload for creating an order.
    pub fn new() -> OrderPostPayload {
        OrderPostPayload {
            fulfillment_status: None,
            internal_notes: None,
            payment_status: None,
            address_id: None,
            customer_id: None,
            notify: None,
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

