/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Order : A payment request for some content or service.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// ID of the order.
    #[serde(rename = "id")]
    pub id: String,
    /// A vanity id to be displayed for the order. For example, \"Order #1000\".
    #[serde(rename = "display_id")]
    pub display_id: i32,
    /// The total price of the order given in cents.
    #[serde(rename = "total_price_cents")]
    pub total_price_cents: i32,
    #[serde(rename = "currency")]
    pub currency: Box<crate::models::Currency>,
    /// The payment status of the order.
    #[serde(rename = "payment_status")]
    pub payment_status: PaymentStatus,
    /// A URL for to pay for the order.
    #[serde(rename = "payment_url", skip_serializing_if = "Option::is_none")]
    pub payment_url: Option<String>,
    #[serde(rename = "listing", skip_serializing_if = "Option::is_none")]
    pub listing: Option<Box<crate::models::PartialListing>>,
    /// The fulfillment status of the order.
    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: FulfillmentStatus,
}

impl Order {
    /// A payment request for some content or service.
    pub fn new(id: String, display_id: i32, total_price_cents: i32, currency: crate::models::Currency, payment_status: PaymentStatus, fulfillment_status: FulfillmentStatus) -> Order {
        Order {
            id,
            display_id,
            total_price_cents,
            currency: Box::new(currency),
            payment_status,
            payment_url: None,
            listing: None,
            fulfillment_status,
        }
    }
}

/// The payment status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "unpaid")]
    Unpaid,
}
/// The fulfillment status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FulfillmentStatus {
    #[serde(rename = "fulfilled")]
    Fulfilled,
    #[serde(rename = "unfulfilled")]
    Unfulfilled,
}

