/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ApiError : A generic error returned by the API.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiError {
    /// A generic error returned by the API.
    pub fn new(message: String) -> ApiError {
        ApiError {
            message,
        }
    }
}


