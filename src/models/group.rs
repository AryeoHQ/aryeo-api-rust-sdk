/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Group : A collection of users that can interact with the Aryeo platform. Permissions and properties are determined based on the group's type which can be creator, agent, or brokerage.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    /// String representing the object’s type. Objects of the same type share the same schema.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// ID of the group. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of the group. Can be CREATOR, AGENT, or BROKERAGE, and may dictate the attributes of the group returned.
    #[serde(rename = "type")]
    pub _type: Type,
    /// The name of the group.
    #[serde(rename = "name")]
    pub name: String,
    /// The email address of a group.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// A phone number represented in whichever standards specified by the group, typically ###-###-#### (separated by hyphens).
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The website URL of a group.
    #[serde(rename = "website_url", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    /// The logo URL of a group.
    #[serde(rename = "logo_url", skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// The profile image URL of a real estate agent. Only returned if group's type is AGENT.
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The name of the brokerage or team of a real estate agent. Only returned if group's type is AGENT.
    #[serde(rename = "office_name", skip_serializing_if = "Option::is_none")]
    pub office_name: Option<String>,
    /// The license number of a real estate agent. Only returned if group's type is AGENT.
    #[serde(rename = "license_number", skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
    #[serde(rename = "social_profiles", skip_serializing_if = "Option::is_none")]
    pub social_profiles: Option<Box<crate::models::SocialProfiles>>,
    #[serde(rename = "default_order_form", skip_serializing_if = "Option::is_none")]
    pub default_order_form: Option<Box<crate::models::OrderForm>>,
    /// An array of order forms a vendor group provides for placing orders. Only returned if group's type is CREATOR. 
    #[serde(rename = "order_forms", skip_serializing_if = "Option::is_none")]
    pub order_forms: Option<Vec<crate::models::OrderForm>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::User>>,
    /// The Aryeo users associated with this group.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    /// Does this group represent a brokerage or an agent who belongs to a brokerage?
    #[serde(rename = "is_brokerage_or_brokerage_agent")]
    pub is_brokerage_or_brokerage_agent: bool,
}

impl Group {
    /// A collection of users that can interact with the Aryeo platform. Permissions and properties are determined based on the group's type which can be creator, agent, or brokerage.
    pub fn new(id: String, _type: Type, name: String, is_brokerage_or_brokerage_agent: bool) -> Group {
        Group {
            object: None,
            id,
            _type,
            name,
            email: None,
            phone: None,
            website_url: None,
            logo_url: None,
            avatar_url: None,
            office_name: None,
            license_number: None,
            social_profiles: None,
            default_order_form: None,
            order_forms: None,
            owner: None,
            users: None,
            is_brokerage_or_brokerage_agent,
        }
    }
}

/// The type of the group. Can be CREATOR, AGENT, or BROKERAGE, and may dictate the attributes of the group returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CREATOR")]
    CREATOR,
    #[serde(rename = "AGENT")]
    AGENT,
    #[serde(rename = "BROKERAGE")]
    BROKERAGE,
}

