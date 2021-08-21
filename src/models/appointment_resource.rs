/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// AppointmentResource : An appointment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppointmentResource {
    /// What was the state of the request?
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::Appointment>>,
}

impl AppointmentResource {
    /// An appointment.
    pub fn new(status: String) -> AppointmentResource {
        AppointmentResource {
            status,
            data: None,
        }
    }
}


