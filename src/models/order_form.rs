/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// OrderForm : A mechanism for placing new orders on the Aryeo platform. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderForm {
    /// ID of the order form. UUID Version 4.
    #[serde(rename = "id")]
    pub id: String,
    /// The title or name of the order form.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A URL of a publicly-accessible webpage for this order form.
    #[serde(rename = "url")]
    pub url: String,
}

impl OrderForm {
    /// A mechanism for placing new orders on the Aryeo platform. 
    pub fn new(id: String, url: String) -> OrderForm {
        OrderForm {
            id,
            title: None,
            url,
        }
    }
}


