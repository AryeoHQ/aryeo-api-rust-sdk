# UnconfirmedAppointment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the appointment. | 
**title** | Option<**String**> | The title of the appointment. | [optional]
**description** | Option<**String**> | The multi-line description of the appointment. | [optional]
**order** | Option<[**crate::models::Order**](Order.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::User>**](User.md)> | Users attached to the appointment. | [optional]
**preference_type** | Option<**String**> | The type of preferred scheduling information provided by a customer to aid in scheduling this appointment. | [optional]
**preferred_start_at** | Option<**String**> | A preferred date and time (ISO 8601 format) for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME.  | [optional]
**preferred_start_at_day** | Option<[**String**](string.md)> | A preferred date (ISO 8601 format) for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME_OF_DAY.  | [optional]
**preferred_start_at_time_of_day** | Option<**String**> | A preferred time of day for when the appointment could start. Only returned if unconfirmed appointment's preference type is TIME_OF_DAY.  | [optional]
**duration** | Option<**i32**> | The estimated length of the appointment in minutes, if available. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


