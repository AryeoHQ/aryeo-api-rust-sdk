/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Group : A collection of users that can interact with the Aryeo platform. Permissions and properties are determined based on the group's type which can be creator, agent, or brokerage.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    /// ID of the group.
    #[serde(rename = "id")]
    pub id: String,
    /// The type of group.
    #[serde(rename = "group_type")]
    pub group_type: GroupType,
    /// The name of the group.
    #[serde(rename = "name")]
    pub name: String,
    /// Group logo.
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// Email.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number.
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Website.
    #[serde(rename = "website", skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// Does this group represent a brokerage or an agent who belongs to a brokerage?
    #[serde(rename = "is_brokerage_or_brokerage_agent")]
    pub is_brokerage_or_brokerage_agent: bool,
    #[serde(rename = "social_profiles", skip_serializing_if = "Option::is_none")]
    pub social_profiles: Option<Box<crate::models::SocialProfiles>>,
    #[serde(rename = "agent_properties", skip_serializing_if = "Option::is_none")]
    pub agent_properties: Option<Box<crate::models::GroupAgentProperties>>,
    /// users
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
    #[serde(rename = "default_order_form", skip_serializing_if = "Option::is_none")]
    pub default_order_form: Option<Box<crate::models::OrderForm>>,
    /// An array of order forms.
    #[serde(rename = "order_forms", skip_serializing_if = "Option::is_none")]
    pub order_forms: Option<Vec<crate::models::OrderForm>>,
}

impl Group {
    /// A collection of users that can interact with the Aryeo platform. Permissions and properties are determined based on the group's type which can be creator, agent, or brokerage.
    pub fn new(id: String, group_type: GroupType, name: String, is_brokerage_or_brokerage_agent: bool) -> Group {
        Group {
            id,
            group_type,
            name,
            logo: None,
            email: None,
            phone: None,
            website: None,
            is_brokerage_or_brokerage_agent,
            social_profiles: None,
            agent_properties: None,
            users: None,
            default_order_form: None,
            order_forms: None,
        }
    }
}

/// The type of group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GroupType {
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "brokerage")]
    Brokerage,
}

