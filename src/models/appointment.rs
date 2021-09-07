/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Appointment : An appointment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Appointment {
    /// The ID of the appointment.
    #[serde(rename = "id")]
    pub id: String,
    /// The status of the appointment.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The title of the appointment.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The multi-line description of the appointment.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The date and time (ISO 8601 format) when the appointment is set to start.
    #[serde(rename = "start_at", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    /// The date and time (ISO 8601 format) when the appointment is set to end.
    #[serde(rename = "end_at", skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    /// The length of the appointment in minutes.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<crate::models::Order>>,
    /// Users attached to the appointment.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    /// Items attached to the appointment.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::OrderItem>>,
}

impl Appointment {
    /// An appointment.
    pub fn new(id: String) -> Appointment {
        Appointment {
            id,
            status: None,
            title: None,
            description: None,
            start_at: None,
            end_at: None,
            duration: None,
            order: None,
            users: None,
            items: None,
        }
    }
}

/// The status of the appointment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "SCHEDULED")]
    SCHEDULED,
    #[serde(rename = "UNSCHEDULED")]
    UNSCHEDULED,
    #[serde(rename = "RESCHEDULED")]
    RESCHEDULED,
    #[serde(rename = "CANCELED")]
    CANCELED,
}

