# OrderPostPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fulfillment_status** | Option<**String**> | The fulfillment status of the order. Defaults to \"UNFULFILLED\". | [optional]
**internal_notes** | Option<**String**> | Internal notes that will be attached to the order. Viewable only by the team. | [optional]
**address_id** | Option<**String**> | ID of the address to associate with the order. UUID Version 4. | [optional]
**customer_id** | Option<**String**> | ID of the customer to associate with the order. UUID Version 4. | [optional]
**notify** | Option<**bool**> | Indicates if the customer and creator notifications should be sent when creating the order. Requires an address and customer to be set in order for the notifications to be sent. | [optional]
**lock_download_for_payment** | Option<**bool**> | Indicates if the downloads for the attached listing should be locked while there is an outstanding balance on the order. | [optional]
**allow_payments_before_fulfillment** | Option<**bool**> | Indicates if the order will allow payments from the customer before the order is marked as fulfilled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


