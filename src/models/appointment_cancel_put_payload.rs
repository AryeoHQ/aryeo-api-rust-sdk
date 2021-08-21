/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// AppointmentCancelPutPayload : Payload for canceling an appointment record.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppointmentCancelPutPayload {
    /// Sends a notification to the appointment's order's customer that the appointment was canceled.
    #[serde(rename = "notify_customer", skip_serializing_if = "Option::is_none")]
    pub notify_customer: Option<bool>,
}

impl AppointmentCancelPutPayload {
    /// Payload for canceling an appointment record.
    pub fn new() -> AppointmentCancelPutPayload {
        AppointmentCancelPutPayload {
            notify_customer: None,
        }
    }
}


