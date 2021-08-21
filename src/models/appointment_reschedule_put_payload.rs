/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// AppointmentReschedulePutPayload : Payload for rescheduling an appointment record.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppointmentReschedulePutPayload {
    /// The new date and time (ISO 8601 format) when the appointment is set to start.
    #[serde(rename = "start_at")]
    pub start_at: Option<String>,
    /// The new date and time (ISO 8601 format) when the appointment is set to end.
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,
    /// Send a notification to the appointment's order's customer that the appointment was rescheduled.
    #[serde(rename = "notify_customer", skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
}

impl AppointmentReschedulePutPayload {
    /// Payload for rescheduling an appointment record.
    pub fn new(start_at: Option<String>, end_at: Option<String>) -> AppointmentReschedulePutPayload {
        AppointmentReschedulePutPayload {
            start_at,
            end_at,
            notify_customer: None,
        }
    }
}


