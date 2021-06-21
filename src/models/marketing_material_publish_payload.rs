/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// MarketingMaterialPublishPayload : Payload for publishing a marketing material record.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketingMaterialPublishPayload {
    /// String representation of a polotno JSON object.
    #[serde(rename = "polotno_json", skip_serializing_if = "Option::is_none")]
    pub polotno_json: Option<String>,
}

impl MarketingMaterialPublishPayload {
    /// Payload for publishing a marketing material record.
    pub fn new() -> MarketingMaterialPublishPayload {
        MarketingMaterialPublishPayload {
            polotno_json: None,
        }
    }
}


