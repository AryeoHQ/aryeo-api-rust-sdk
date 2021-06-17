/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PropertyWebsites : Websites that displays information about a property.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyWebsites {
    /// URL for website.
    #[serde(rename = "branded_url")]
    pub branded_url: String,
    /// URL for website.
    #[serde(rename = "unbranded_url")]
    pub unbranded_url: String,
    /// ID for property website
    #[serde(rename = "id")]
    pub id: i32,
}

impl PropertyWebsites {
    /// Websites that displays information about a property.
    pub fn new(branded_url: String, unbranded_url: String, id: i32) -> PropertyWebsites {
        PropertyWebsites {
            branded_url,
            unbranded_url,
            id,
        }
    }
}


