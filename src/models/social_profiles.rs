/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// SocialProfiles : Details for a real estate agent.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialProfiles {
    /// URL for Facebook.
    #[serde(rename = "facebook_profile_url", skip_serializing_if = "Option::is_none")]
    pub facebook_profile_url: Option<String>,
    /// URL for Instagram.
    #[serde(rename = "instagram_profile_url", skip_serializing_if = "Option::is_none")]
    pub instagram_profile_url: Option<String>,
    /// URL for Twitter.
    #[serde(rename = "twitter_profile_url", skip_serializing_if = "Option::is_none")]
    pub twitter_profile_url: Option<String>,
    /// URL for LinkedIn.
    #[serde(rename = "linkedin_profile_url", skip_serializing_if = "Option::is_none")]
    pub linkedin_profile_url: Option<String>,
    /// URL for Zillow.
    #[serde(rename = "zillow_profile_url", skip_serializing_if = "Option::is_none")]
    pub zillow_profile_url: Option<String>,
}

impl SocialProfiles {
    /// Details for a real estate agent.
    pub fn new() -> SocialProfiles {
        SocialProfiles {
            facebook_profile_url: None,
            instagram_profile_url: None,
            twitter_profile_url: None,
            linkedin_profile_url: None,
            zillow_profile_url: None,
        }
    }
}


