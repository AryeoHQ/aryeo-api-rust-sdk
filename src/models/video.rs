/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Video : A recording of moving visual images. Provided either as a download URL (MP4) or a link (e.g. YouTube, Vimeo).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// ID of the video. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The title of the video given by the uploading user.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The video's runtime in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// The display type determines if the video is branded or unbranded (can also be none or both). This affects whether the video is displayed on branded or unbranded marketing materials such as the property website.
    #[serde(rename = "display_type")]
    pub display_type: DisplayType,
    /// The original upload source of the video, used to determine how to handle the playback_url of the video and other display properties. 
    #[serde(rename = "source_type")]
    pub source_type: SourceType,
    /// A thumbnail image URL for the video.
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: String,
    /// A URL linking to playback stream of the video.
    #[serde(rename = "playback_url")]
    pub playback_url: String,
    /// A URL for downloading the video.
    #[serde(rename = "download_url", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// A URL linking to a public viewing optimized webpage the video.
    #[serde(rename = "share_url", skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
}

impl Video {
    /// A recording of moving visual images. Provided either as a download URL (MP4) or a link (e.g. YouTube, Vimeo).
    pub fn new(id: String, display_type: DisplayType, source_type: SourceType, thumbnail_url: String, playback_url: String) -> Video {
        Video {
            id,
            title: None,
            duration: None,
            display_type,
            source_type,
            thumbnail_url,
            playback_url,
            download_url: None,
            share_url: None,
        }
    }
}

/// The display type determines if the video is branded or unbranded (can also be none or both). This affects whether the video is displayed on branded or unbranded marketing materials such as the property website.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "BRANDED")]
    BRANDED,
    #[serde(rename = "UNBRANDED")]
    UNBRANDED,
    #[serde(rename = "BOTH")]
    BOTH,
    #[serde(rename = "NONE")]
    NONE,
}
/// The original upload source of the video, used to determine how to handle the playback_url of the video and other display properties. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "YOUTUBE")]
    YOUTUBE,
    #[serde(rename = "VIMEO")]
    VIMEO,
    #[serde(rename = "OPTIMIZED")]
    OPTIMIZED,
    #[serde(rename = "UPLOADED")]
    UPLOADED,
    #[serde(rename = "LINK")]
    LINK,
}

