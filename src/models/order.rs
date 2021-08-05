/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Order : A payment request for some content or service.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// ID of the order. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// A vanity id used for internal tracking of orders for a given vendor. 
    #[serde(rename = "number")]
    pub number: i32,
    /// The title of the order, generated by combining the order's number property with the prefix \"Order #\". 
    #[serde(rename = "title")]
    pub title: String,
    /// The fulfillment status of the order.
    #[serde(rename = "fulfillment_status")]
    pub fulfillment_status: FulfillmentStatus,
    /// The payment status of the order.
    #[serde(rename = "payment_status")]
    pub payment_status: PaymentStatus,
    /// The three-letter ISO 4217 currency code for the currency in which this order was or will be transacted. Must be a supported currency of Aryeo. 
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// A positive integer in cents representing the total order amount that was or will be charged.
    #[serde(rename = "total_amount", skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i32>,
    /// A URL of a publicly-accessible webpage to pay for the order.
    #[serde(rename = "payment_url", skip_serializing_if = "Option::is_none")]
    pub payment_url: Option<String>,
    /// A URL of a publicly-accessible webpage to see the order's status.
    #[serde(rename = "status_url", skip_serializing_if = "Option::is_none")]
    pub status_url: Option<String>,
}

impl Order {
    /// A payment request for some content or service.
    pub fn new(id: String, number: i32, title: String, fulfillment_status: FulfillmentStatus, payment_status: PaymentStatus) -> Order {
        Order {
            id,
            number,
            title,
            fulfillment_status,
            payment_status,
            currency: None,
            total_amount: None,
            payment_url: None,
            status_url: None,
        }
    }
}

/// The fulfillment status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FulfillmentStatus {
    #[serde(rename = "FULFILLED")]
    FULFILLED,
    #[serde(rename = "UNFULFILLED")]
    UNFULFILLED,
}
/// The payment status of the order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "PAID")]
    PAID,
    #[serde(rename = "UNPAID")]
    UNPAID,
}
/// The three-letter ISO 4217 currency code for the currency in which this order was or will be transacted. Must be a supported currency of Aryeo. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "CAD")]
    CAD,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "CHF")]
    CHF,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "AUD")]
    AUD,
    #[serde(rename = "NZD")]
    NZD,
    #[serde(rename = "ZAR")]
    ZAR,
}

