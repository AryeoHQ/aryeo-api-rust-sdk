# PartialListing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the listing. | 
**address** | [**crate::models::PartialAddress**](PartialAddress.md) |  | 
**delivery_status** | **String** | Has the listing been delivered? | 
**thumbnail_url** | Option<**String**> | Thumbnail URL for the listing. | [optional]
**price** | Option<**i32**> | The price of the property in dollars. | [optional]
**branded_url** | Option<**String**> | URL for branded property website. | [optional]
**square_feet** | Option<**f32**> | Total number of square feet. | [optional]
**bedrooms** | Option<**i32**> | Number of bedrooms. | [optional]
**bathrooms** | Option<**f32**> | Number of bathrooms. | [optional]
**downloads_enabled** | **bool** | Are downloads enabled for this listing? | 
**status** | Option<**String**> | Sales status for the listing. | [optional]
**property_details** | Option<[**crate::models::PropertyDetails**](PropertyDetails.md)> |  | [optional]
**agent** | Option<[**crate::models::PartialGroup**](PartialGroup.md)> |  | [optional]
**co_agent** | Option<[**crate::models::PartialGroup**](PartialGroup.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


