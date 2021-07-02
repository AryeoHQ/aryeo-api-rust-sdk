/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// MarketingMaterialTemplatePublishPayload : Payload for publishing a marketing material template record.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketingMaterialTemplatePublishPayload {
    /// String representation of a polotno JSON object.
    #[serde(rename = "polotno_json", skip_serializing_if = "Option::is_none")]
    pub polotno_json: Option<String>,
}

impl MarketingMaterialTemplatePublishPayload {
    /// Payload for publishing a marketing material template record.
    pub fn new() -> MarketingMaterialTemplatePublishPayload {
        MarketingMaterialTemplatePublishPayload {
            polotno_json: None,
        }
    }
}


