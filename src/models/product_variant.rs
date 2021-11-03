/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ProductVariant : A variant of a product.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductVariant {
    /// ID of the product variant. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The title of the product variant.
    #[serde(rename = "title")]
    pub title: String,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00) representing the price of the product variant.
    #[serde(rename = "price")]
    pub price: i32,
    /// The duration of the product item, in minutes.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

impl ProductVariant {
    /// A variant of a product.
    pub fn new(id: String, title: String, price: i32) -> ProductVariant {
        ProductVariant {
            id,
            title,
            price,
            duration: None,
        }
    }
}


