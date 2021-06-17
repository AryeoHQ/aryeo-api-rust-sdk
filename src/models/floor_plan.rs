/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// FloorPlan : A scale diagram of the arrangement of a building.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FloorPlan {
    /// ID of the floor plan.
    #[serde(rename = "id")]
    pub id: i32,
    /// A URL for a thumbnail-sized version of the floor plan.
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    /// A URL for a large version of the floor plan.
    #[serde(rename = "large_url")]
    pub large_url: String,
    /// A URL for the original, full-resolution version of the floor plan. Useful for downloading.
    #[serde(rename = "original_url")]
    pub original_url: String,
    /// The title (or caption) of the floor plan.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Index order position of the floor plan.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

impl FloorPlan {
    /// A scale diagram of the arrangement of a building.
    pub fn new(id: i32, thumbnail_url: String, large_url: String, original_url: String) -> FloorPlan {
        FloorPlan {
            id,
            thumbnail_url,
            large_url,
            original_url,
            title: None,
            index: None,
        }
    }
}


