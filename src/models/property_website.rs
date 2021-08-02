/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PropertyWebsite : Website (in branded and unbranded versions) that displays information about a property.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyWebsite {
    /// ID of the website.
    #[serde(rename = "id")]
    pub id: String,
    /// URL for branded version of website.
    #[serde(rename = "branded_url")]
    pub branded_url: String,
    /// URL for unbranded version of website.
    #[serde(rename = "unbranded_url")]
    pub unbranded_url: String,
}

impl PropertyWebsite {
    /// Website (in branded and unbranded versions) that displays information about a property.
    pub fn new(id: String, branded_url: String, unbranded_url: String) -> PropertyWebsite {
        PropertyWebsite {
            id,
            branded_url,
            unbranded_url,
        }
    }
}


