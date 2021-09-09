/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// UnconfirmedAppointmentResource : An appointment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnconfirmedAppointmentResource {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::UnconfirmedAppointment>>,
}

impl UnconfirmedAppointmentResource {
    /// An appointment.
    pub fn new(status: String) -> UnconfirmedAppointmentResource {
        UnconfirmedAppointmentResource {
            status,
            data: None,
        }
    }
}


