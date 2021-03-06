/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// TimeslotCollection : A collection of bookable timeslots.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeslotCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Timeslot>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl TimeslotCollection {
    /// A collection of bookable timeslots.
    pub fn new(status: String) -> TimeslotCollection {
        TimeslotCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


