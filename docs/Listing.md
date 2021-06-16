# Listing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the listing. | 
**address** | [**crate::models::PartialAddress**](PartialAddress.md) |  | 
**delivery_status** | **String** | Has this listing been delivered? | 
**thumbnail_url** | Option<**String**> | Thumbnail URL for the listing. | [optional]
**agent** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**co_agent** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**images** | Option<[**Vec<crate::models::Image>**](Image.md)> | images | [optional]
**videos** | Option<[**Vec<crate::models::Video>**](Video.md)> | videos | [optional]
**floor_plans** | Option<[**Vec<crate::models::FloorPlan>**](FloorPlan.md)> | floor_plans | [optional]
**property_websites** | Option<[**crate::models::PropertyWebsites**](PropertyWebsites.md)> |  | [optional]
**interactive_content** | Option<[**Vec<crate::models::InteractiveContent>**](InteractiveContent.md)> | interactive_content | [optional]
**property_details** | Option<[**crate::models::PropertyDetails**](PropertyDetails.md)> |  | [optional]
**downloads_enabled** | **bool** | Are downloads enabled for this listing? | 
**orders** | Option<[**Vec<crate::models::Order>**](Order.md)> | orders | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


