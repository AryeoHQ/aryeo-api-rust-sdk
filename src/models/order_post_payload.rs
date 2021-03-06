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
    /// ID of the address to associate with the order. UUID Version 4.
    #[serde(rename = "address_id", skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    /// ID of the customer to associate with the order. UUID Version 4.
    #[serde(rename = "customer_id", skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Indicates if the customer and creator notifications should be sent when creating the order. Requires an address and customer to be set in order for the notifications to be sent.
    #[serde(rename = "notify", skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    /// Indicates if the downloads for the attached listing should be locked while there is an outstanding balance on the order.
    #[serde(rename = "lock_download_for_payment", skip_serializing_if = "Option::is_none")]
    pub lock_download_for_payment: Option<bool>,
    /// Indicates if the order will allow payments from the customer before the order is marked as fulfilled.
    #[serde(rename = "allow_payments_before_fulfillment", skip_serializing_if = "Option::is_none")]
    pub allow_payments_before_fulfillment: Option<bool>,
}

impl OrderPostPayload {
    /// Payload for creating an order.
    pub fn new() -> OrderPostPayload {
        OrderPostPayload {
            fulfillment_status: None,
            internal_notes: None,
            address_id: None,
            customer_id: None,
            notify: None,
            lock_download_for_payment: None,
            allow_payments_before_fulfillment: None,
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

