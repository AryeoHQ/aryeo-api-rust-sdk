/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// CalendarDay : A bookable time range with available users.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalendarDay {
    /// Calendar day that has available timeslots.
    #[serde(rename = "date")]
    pub date: String,
    /// Does the day have availability?
    #[serde(rename = "is_available")]
    pub is_available: bool,
}

impl CalendarDay {
    /// A bookable time range with available users.
    pub fn new(date: String, is_available: bool) -> CalendarDay {
        CalendarDay {
            date,
            is_available,
        }
    }
}


