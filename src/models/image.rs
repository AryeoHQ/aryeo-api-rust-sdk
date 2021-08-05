/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Image : A visual representation of something.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    /// ID of the image. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// A URL for a thumbnail-sized version of the image.
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    /// A URL for a large version of the image.
    #[serde(rename = "large_url")]
    pub large_url: String,
    /// A URL for the original, full-resolution version of the image. Useful for downloading.
    #[serde(rename = "original_url")]
    pub original_url: String,
    /// The order in which the image should be displayed amongst other related images.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// A brief explanation of the image.
    #[serde(rename = "caption", skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Should the image be displayed in a gallery?
    #[serde(rename = "display_in_gallery")]
    pub display_in_gallery: bool,
}

impl Image {
    /// A visual representation of something.
    pub fn new(id: String, thumbnail_url: String, large_url: String, original_url: String, display_in_gallery: bool) -> Image {
        Image {
            id,
            thumbnail_url,
            large_url,
            original_url,
            index: None,
            caption: None,
            display_in_gallery,
        }
    }
}


