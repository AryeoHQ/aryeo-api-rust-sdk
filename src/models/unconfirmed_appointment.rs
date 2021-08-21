/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// UnconfirmedAppointment : An unconfirmed appointment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnconfirmedAppointment {
    /// The ID of the appointment.
    #[serde(rename = "id")]
    pub id: String,
    /// The title of the appointment.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The multi-line description of the appointment.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<crate::models::Order>>,
    /// Users attached to the appointment.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    /// The type of preferred scheduling information provided by a customer to aid in scheduling this appointment.
    #[serde(rename = "preference_type", skip_serializing_if = "Option::is_none")]
    pub preference_type: Option<PreferenceType>,
    /// A preferred date and time (ISO 8601 format) for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME. 
    #[serde(rename = "preferred_start_at", skip_serializing_if = "Option::is_none")]
    pub preferred_start_at: Option<String>,
    /// A preferred date (ISO 8601 format) for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME_OF_DAY. 
    #[serde(rename = "preferred_start_at_day", skip_serializing_if = "Option::is_none")]
    pub preferred_start_at_day: Option<String>,
    /// A preferred time of day for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME_OF_DAY. 
    #[serde(rename = "preferred_start_at_time_of_day", skip_serializing_if = "Option::is_none")]
    pub preferred_start_at_time_of_day: Option<PreferredStartAtTimeOfDay>,
    /// The estimated length of the appointment in minutes, if available.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

impl UnconfirmedAppointment {
    /// An unconfirmed appointment.
    pub fn new(id: String) -> UnconfirmedAppointment {
        UnconfirmedAppointment {
            id,
            title: None,
            description: None,
            order: None,
            users: None,
            preference_type: None,
            preferred_start_at: None,
            preferred_start_at_day: None,
            preferred_start_at_time_of_day: None,
            duration: None,
        }
    }
}

/// The type of preferred scheduling information provided by a customer to aid in scheduling this appointment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PreferenceType {
    #[serde(rename = "ASAP")]
    ASAP,
    #[serde(rename = "TIME")]
    TIME,
    #[serde(rename = "TIME_OF_DAY")]
    TIMEOFDAY,
}
/// A preferred time of day for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME_OF_DAY. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PreferredStartAtTimeOfDay {
    #[serde(rename = "MORNING")]
    MORNING,
    #[serde(rename = "AFTERNOON")]
    AFTERNOON,
    #[serde(rename = "TWILIGHT")]
    TWILIGHT,
}

