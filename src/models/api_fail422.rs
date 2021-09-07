/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ApiFail422 : A processing or validation failure returned by the API.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiFail422 {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
}

impl ApiFail422 {
    /// A processing or validation failure returned by the API.
    pub fn new(status: String) -> ApiFail422 {
        ApiFail422 {
            status,
        }
    }
}


