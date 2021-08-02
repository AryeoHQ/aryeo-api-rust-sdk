/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ApiFail : A generic failure returned by the API.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiFail {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
}

impl ApiFail {
    /// A generic failure returned by the API.
    pub fn new(status: String) -> ApiFail {
        ApiFail {
            status,
        }
    }
}


