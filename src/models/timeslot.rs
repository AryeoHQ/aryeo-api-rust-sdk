/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Timeslot : A bookable time range, including users that are available.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Timeslot {
    /// Start time of the available slot
    #[serde(rename = "start_at")]
    pub start_at: String,
    /// End time of the available slot
    #[serde(rename = "end_at")]
    pub end_at: String,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
}

impl Timeslot {
    /// A bookable time range, including users that are available.
    pub fn new(start_at: String, end_at: String) -> Timeslot {
        Timeslot {
            start_at,
            end_at,
            users: None,
        }
    }
}


