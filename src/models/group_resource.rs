/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// GroupResource : A group.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupResource {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::Group>>,
}

impl GroupResource {
    /// A group.
    pub fn new(status: String) -> GroupResource {
        GroupResource {
            status,
            data: None,
        }
    }
}


