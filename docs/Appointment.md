# Appointment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the appointment. | 
**status** | Option<**String**> | The status of the appointment. | [optional]
**title** | Option<**String**> | The title of the appointment. | [optional]
**description** | Option<**String**> | The multi-line description of the appointment. | [optional]
**start_at** | Option<**String**> | The date and time (ISO 8601 format) when the appointment is set to start. | [optional]
**end_at** | Option<**String**> | The date and time (ISO 8601 format) when the appointment is set to end. | [optional]
**duration** | Option<**i32**> | The length of the appointment in minutes. | [optional]
**order** | Option<[**crate::models::Order**](Order.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::User>**](User.md)> | Users attached to the appointment. | [optional]
**items** | Option<[**Vec<crate::models::OrderItem>**](OrderItem.md)> | Items attached to the appointment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


