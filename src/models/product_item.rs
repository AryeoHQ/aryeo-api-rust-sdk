/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ProductItem : A subtype or part of a product that a group is selling.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductItem {
    /// ID of the product item.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ProductItem {
    /// A subtype or part of a product that a group is selling.
    pub fn new() -> ProductItem {
        ProductItem {
            id: None,
        }
    }
}


