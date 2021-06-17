/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PartialGroup : A collection of users that can interact with the Aryeo platform. Permissions and properties are determined based on the type which can be creator, agent, or brokerage.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialGroup {
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
    /// Does this group represent a brokerage or an agent who belongs to a brokerage?
    #[serde(rename = "is_brokerage_or_brokerage_agent")]
    pub is_brokerage_or_brokerage_agent: bool,
}

impl PartialGroup {
    /// A collection of users that can interact with the Aryeo platform. Permissions and properties are determined based on the type which can be creator, agent, or brokerage.
    pub fn new(id: String, group_type: GroupType, name: String, is_brokerage_or_brokerage_agent: bool) -> PartialGroup {
        PartialGroup {
            id,
            group_type,
            name,
            logo: None,
            email: None,
            phone: None,
            is_brokerage_or_brokerage_agent,
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

