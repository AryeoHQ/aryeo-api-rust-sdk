/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// Currency : A system of money used for payment.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    /// The ID of the currency.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the currency.
    #[serde(rename = "name")]
    pub name: String,
    /// The currency symbol.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Is this currency enabled for Aryeo?
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl Currency {
    /// A system of money used for payment.
    pub fn new(id: String, name: String, symbol: String, enabled: bool) -> Currency {
        Currency {
            id,
            name,
            symbol,
            enabled,
        }
    }
}


