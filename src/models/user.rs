/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// User : A record of a person on the Aryeo platform.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// UUID of the user.
    #[serde(rename = "id")]
    pub id: String,
    /// Avatar.
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Email.
    #[serde(rename = "email")]
    pub email: String,
    /// First name.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Timezone.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Describes user's relationship (access level) to a specified group.
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
}

impl User {
    /// A record of a person on the Aryeo platform.
    pub fn new(id: String, email: String) -> User {
        User {
            id,
            avatar: None,
            email,
            first_name: None,
            last_name: None,
            timezone: None,
            phone: None,
            relationship: None,
        }
    }
}


