/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// CalendarDayCollection : A collection of calendar days that have available timeslots



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarDayCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::CalendarDay>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl CalendarDayCollection {
    /// A collection of calendar days that have available timeslots
    pub fn new(status: String) -> CalendarDayCollection {
        CalendarDayCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


