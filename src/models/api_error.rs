/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ApiError : A generic error returned by the API.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
    /// A numeric code corresponding to the error.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

impl ApiError {
    /// A generic error returned by the API.
    pub fn new(status: String, message: String) -> ApiError {
        ApiError {
            status,
            message,
            code: None,
        }
    }
}


