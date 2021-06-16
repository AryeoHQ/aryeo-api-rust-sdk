# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the order. | 
**display_id** | **i32** | A vanity id to be displayed for the order. For example, \"Order #1000\". | 
**total_price_cents** | **i32** | The total price of the order given in cents. | 
**currency** | [**crate::models::Currency**](Currency.md) |  | 
**payment_status** | **String** | The payment status of the order. | 
**payment_url** | Option<**String**> | A URL for to pay for the order. | [optional]
**listing** | Option<[**crate::models::PartialListing**](PartialListing.md)> |  | [optional]
**fulfillment_status** | **String** | The fulfillment status of the order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


