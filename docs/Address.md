# Address

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the address. UUID Version 4. | 
**latitude** | **f32** | The geographic latitude of some reference point of the location, specified in degrees and decimal parts. Positive numbers must not include the plus symbol. | 
**longitude** | **f32** | The geographic longitude of some reference point of the location, specified in degrees and decimal parts. Positive numbers must not include the plus symbol. | 
**street_number** | Option<**String**> | The street number portion of a location's address. In some areas, the street number may contain non-numeric characters. This field can also contain extensions and modifiers to the street number, such as \"1/2\" or \"-B\". | [optional]
**street_name** | Option<**String**> | The street name portion of a location's address. | [optional]
**unit_number** | Option<**String**> | The number or portion of a larger building or complex. Examples are: \"APT G\", \"55\", etc. | [optional]
**postal_code** | Option<**String**> | The postal code portion of a location's address. | [optional]
**city** | Option<**String**> | The city of a location's address. | [optional]
**city_region** | Option<**String**> | A sub-section or area of a defined city. Examples would be SoHo in New York, NY, Ironbound in Newark, NJ or Inside the Beltway. | [optional]
**county_or_parish** | Option<**String**> | The County, Parish or other regional authority of the location. | [optional]
**state_or_province** | Option<**String**> | The ISO 3166-2 subdivision code for the state or province of the location. For example, “MA” for Massachusetts, United States. | [optional]
**state_or_province_region** | Option<**String**> | A sub-section or area of a defined state or province. Examples would be the Keys in FL or Hudson Valley in NY. | [optional]
**country** | Option<**String**> | The ISO 3166-1 country code for this for the country of the location. | [optional]
**country_region** | Option<**String**> | A sub-section or area of a defined country. Examples would be Napa Valley in the US, or the Amalfi Coast in Italy. | [optional]
**unparsed_address** | Option<**String**> | Unparsed text representation of the address.  | [optional]
**unparsed_address_part_one** | Option<**String**> | Unparsed text representation of the first part of the address, typically including the street number, street name, and unit number if applicable.   | [optional]
**unparsed_address_part_two** | Option<**String**> | Unparsed text representation of the second part of the address, typically including the city, state or province, and postal code.   | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


