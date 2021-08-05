/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// User : A record of a person on the Aryeo platform.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// ID of the user. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: String,
    /// First name of the user.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// A phone number represented in whichever standards specified by the user, typically ###-###-#### (separated by hyphens).
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The avatar image URL of a user.
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Describes user's relationship (access level) to a specified group. Only returned if this resource is returned as a sub-resource of a group.
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
}

impl User {
    /// A record of a person on the Aryeo platform.
    pub fn new(id: String, email: String) -> User {
        User {
            id,
            email,
            first_name: None,
            last_name: None,
            phone: None,
            avatar_url: None,
            relationship: None,
        }
    }
}


