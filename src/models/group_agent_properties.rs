/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// GroupAgentProperties : A subset of group properties relevant to agents.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAgentProperties {
    /// Name of brokerage.
    #[serde(rename = "brokerage_name", skip_serializing_if = "Option::is_none")]
    pub brokerage_name: Option<String>,
    /// Agent's license number.
    #[serde(rename = "agent_license_number", skip_serializing_if = "Option::is_none")]
    pub agent_license_number: Option<String>,
    /// Agent's profile image URL.
    #[serde(rename = "agent_avatar", skip_serializing_if = "Option::is_none")]
    pub agent_avatar: Option<String>,
}

impl GroupAgentProperties {
    /// A subset of group properties relevant to agents.
    pub fn new() -> GroupAgentProperties {
        GroupAgentProperties {
            brokerage_name: None,
            agent_license_number: None,
            agent_avatar: None,
        }
    }
}


