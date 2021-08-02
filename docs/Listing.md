# Listing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the listing. | 
**address** | [**crate::models::Address**](Address.md) |  | 
**mls_number** | Option<**String**> | The identifier for a listing on its local MLS.  | [optional]
**_type** | Option<**String**> | General type of the listing, primarily categorizing its use case. Examples include residential and commercial.  | [optional]
**sub_type** | Option<**String**> | Further specifies the listing type. Examples include family residence and condominium. | [optional]
**status** | Option<**String**> | Local, regional, or otherwise custom status for the listing used by the parties involved in the listing transaction. While variable, these statuses are typically mapped to the listing's standard status. | [optional]
**standard_status** | Option<**String**> | The status of the listing as it reflects the state of the contract between the listing agent and seller or an agreement with a buyer, including Active, Active Under Contract, Canceled, Closed, Expired, Pending, and Withdrawn. | [optional]
**description** | Option<**String**> | Description of the selling points of the building and/or land for sale.  | [optional]
**lot** | Option<[**crate::models::ListingLot**](ListingLot.md)> |  | [optional]
**building** | Option<[**crate::models::ListingBuilding**](ListingBuilding.md)> |  | [optional]
**price** | Option<[**crate::models::ListingPrice**](ListingPrice.md)> |  | [optional]
**list_agent** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**co_list_agent** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**images** | Option<[**Vec<crate::models::Image>**](Image.md)> | images | [optional]
**videos** | Option<[**Vec<crate::models::Video>**](Video.md)> | videos | [optional]
**floor_plans** | Option<[**Vec<crate::models::FloorPlan>**](FloorPlan.md)> | floor_plans | [optional]
**interactive_content** | Option<[**Vec<crate::models::InteractiveContent>**](InteractiveContent.md)> | interactive_content | [optional]
**property_website** | Option<[**crate::models::PropertyWebsite**](PropertyWebsite.md)> |  | [optional]
**orders** | Option<[**Vec<crate::models::Order>**](Order.md)> | orders | [optional]
**downloads_enabled** | **bool** | Are downloads enabled for this listing? | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


