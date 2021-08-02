/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ListingPrice : Valuation data relating to the price of a listing.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListingPrice {
    /// The current price of the listing.
    #[serde(rename = "list_price", skip_serializing_if = "Option::is_none")]
    pub list_price: Option<i32>,
}

impl ListingPrice {
    /// Valuation data relating to the price of a listing.
    pub fn new() -> ListingPrice {
        ListingPrice {
            list_price: None,
        }
    }
}


