/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ProductCategory : A category for products.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductCategory {
    /// ID of the product category. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The title of the product category.
    #[serde(rename = "title")]
    pub title: String,
}

impl ProductCategory {
    /// A category for products.
    pub fn new(id: String, title: String) -> ProductCategory {
        ProductCategory {
            id,
            title,
        }
    }
}


