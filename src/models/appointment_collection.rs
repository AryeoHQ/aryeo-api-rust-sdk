/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// AppointmentCollection : A collection of appointments.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppointmentCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Appointment>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl AppointmentCollection {
    /// A collection of appointments.
    pub fn new(status: String) -> AppointmentCollection {
        AppointmentCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


