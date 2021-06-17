/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// PaginationLinks : Related links for a paginated response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationLinks {
    /// The first page.
    #[serde(rename = "first")]
    pub first: String,
    /// The last page.
    #[serde(rename = "last")]
    pub last: String,
    /// The previous page. This is specified as either `string` or `null` purely for contract testing purposes. The model which is autogenerated from this definition will be thrown out and written by-hand.
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// The next page. This is specified as either `string` or `null` purely for contract testing purposes. The model which is autogenerated from this definition will be thrown out and written by-hand.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl PaginationLinks {
    /// Related links for a paginated response.
    pub fn new(first: String, last: String) -> PaginationLinks {
        PaginationLinks {
            first,
            last,
            prev: None,
            next: None,
        }
    }
}


