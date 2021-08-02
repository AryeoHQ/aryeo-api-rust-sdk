/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */

/// ListingLot : Parcel data of the lot of a listing. Includes nearly everything about the land that makes up the listing.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListingLot {
    /// Total area of the lot of a listing in acres. 
    #[serde(rename = "size_acres", skip_serializing_if = "Option::is_none")]
    pub size_acres: Option<f32>,
    /// Number of parking spaces.
    #[serde(rename = "open_parking_spaces", skip_serializing_if = "Option::is_none")]
    pub open_parking_spaces: Option<f32>,
}

impl ListingLot {
    /// Parcel data of the lot of a listing. Includes nearly everything about the land that makes up the listing.
    pub fn new() -> ListingLot {
        ListingLot {
            size_acres: None,
            open_parking_spaces: None,
        }
    }
}


