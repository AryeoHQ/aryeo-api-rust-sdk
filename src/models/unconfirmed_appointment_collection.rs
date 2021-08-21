/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// UnconfirmedAppointmentCollection : A collection of appointments.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnconfirmedAppointmentCollection {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    /// 
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::UnconfirmedAppointment>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::PaginationMeta>>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Box<crate::models::PaginationLinks>>,
}

impl UnconfirmedAppointmentCollection {
    /// A collection of appointments.
    pub fn new(status: String) -> UnconfirmedAppointmentCollection {
        UnconfirmedAppointmentCollection {
            status,
            data: None,
            meta: None,
            links: None,
        }
    }
}


