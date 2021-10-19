/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Product : A product available for purchase via an order.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    /// ID of the product. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The title of the product.
    #[serde(rename = "title")]
    pub title: String,
    /// The description of the product.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The active status of a product.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The type of product.
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "variants", skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<crate::models::ProductVariant>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<crate::models::ProductCategory>>,
}

impl Product {
    /// A product available for purchase via an order.
    pub fn new(id: String, title: String, _type: Type) -> Product {
        Product {
            id,
            title,
            description: None,
            active: None,
            _type,
            variants: None,
            categories: None,
        }
    }
}

/// The type of product.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MAIN")]
    MAIN,
    #[serde(rename = "ADDON")]
    ADDON,
}

