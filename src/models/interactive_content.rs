/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// InteractiveContent : A 3D virtual tour.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractiveContent {
    /// ID of the content.
    #[serde(rename = "id")]
    pub id: String,
    /// Is the content branded, unbranded, or both?
    #[serde(rename = "display_type")]
    pub display_type: DisplayType,
    /// The type of interactive content.
    #[serde(rename = "content_type")]
    pub content_type: ContentType,
    /// URL for the content.
    #[serde(rename = "url")]
    pub url: String,
    /// A URL for a thumbnail-sized preview of the content.
    #[serde(rename = "thumbnail_url", skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
}

impl InteractiveContent {
    /// A 3D virtual tour.
    pub fn new(id: String, display_type: DisplayType, content_type: ContentType, url: String) -> InteractiveContent {
        InteractiveContent {
            id,
            display_type,
            content_type,
            url,
            thumbnail_url: None,
        }
    }
}

/// Is the content branded, unbranded, or both?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "BRANDED")]
    BRANDED,
    #[serde(rename = "UNBRANDED")]
    UNBRANDED,
    #[serde(rename = "BOTH")]
    BOTH,
}
/// The type of interactive content.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "MATTERPORT")]
    MATTERPORT,
    #[serde(rename = "OTHER")]
    OTHER,
}

