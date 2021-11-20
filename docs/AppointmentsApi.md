# \AppointmentsApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_appointments**](AppointmentsApi.md#get_appointments) | **GET** /appointments | List all appointments.
[**get_available_dates**](AppointmentsApi.md#get_available_dates) | **GET** /scheduling/available-dates | Fetch available days for a user or group
[**get_available_timeslots**](AppointmentsApi.md#get_available_timeslots) | **GET** /scheduling/available-timeslots | Fetch available timeslots for a user or group
[**get_unconfirmed_appointments**](AppointmentsApi.md#get_unconfirmed_appointments) | **GET** /unconfirmed-appointments | List all unconfirmed appointments.
[**get_unconfirmed_appointments_id**](AppointmentsApi.md#get_unconfirmed_appointments_id) | **GET** /unconfirmed-appointments/{unconfirmed_appointment_id} | Retrieve an unconfirmed appointment.
[**put_appointments_appointment_id_cancel**](AppointmentsApi.md#put_appointments_appointment_id_cancel) | **PUT** /appointments/{appointment_id}/cancel | Cancel an appointment.
[**put_appointments_appointment_id_reschedule**](AppointmentsApi.md#put_appointments_appointment_id_reschedule) | **PUT** /appointments/{appointment_id}/reschedule | Reschedule an appointment.



## get_appointments

> crate::models::AppointmentCollection get_appointments(include, filter_tense, filter_start_at_gte, filter_start_at_lte, filter_user_ids, sort, per_page, page)
List all appointments.

List all appointments. By default, returns a list of appointments that have been scheduled and have not been canceled

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |
**filter_tense** | Option<**String**> | Return appointments that are upcoming or in the past. |  |
**filter_start_at_gte** | Option<**String**> | Return appointments where the start_at field is greater than or equal to this value. Effectively, appointments that start after this date. |  |
**filter_start_at_lte** | Option<**String**> | Return appointments where the start_at field is less than or equal to this value. Effectively, appointments that start before this date. |  |
**filter_user_ids** | Option<[**Vec<String>**](String.md)> | The IDs of users whose appointments will be retrieved. UUID Version 4. |  |
**sort** | Option<**String**> | Comma separated list of fields used for sorting. Placing a minus symbol in front of a field name sorts in descending order. Defaults to `-start_at`. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |

### Return type

[**crate::models::AppointmentCollection**](AppointmentCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_dates

> crate::models::CalendarDayCollection get_available_dates(filter_user_ids, filter_appointment_id, filter_start_at, filter_end_at, filter_timeframe, duration, interval, timezone, page, per_page)
Fetch available days for a user or group

Fetch available calendar days for scheduling or rescheduling an appointment. Availability can be retrieved using a specific start & end date range, or using a timeframe. When using a timeframe, the page parameter can be used to flip through weeks, months, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_user_ids** | Option<[**Vec<String>**](String.md)> | The IDs of users whose availability will be retrieved. UUID Version 4. |  |
**filter_appointment_id** | Option<**String**> | Appointment ID used to fetch availability for an existing order |  |
**filter_start_at** | Option<**String**> | Returns availability after start_at |  |
**filter_end_at** | Option<**String**> | Returns availability before end_at |  |
**filter_timeframe** | Option<[**Vec<String>**](String.md)> | Returns availability for a specific timeframe. Used instead of start_at & end_at |  |
**duration** | Option<**i32**> | Duration of the event to schedule. Required if appointment_id isn't specified |  |
**interval** | Option<**i32**> | Interval of bookable timeslots starting at x minutes on the hour . Required if appointment_id isn't specified |  |
**timezone** | Option<**String**> | Timezone of the client. Localizes the available days |  |
**page** | Option<**i32**> | The requested page of results |  |
**per_page** | Option<**i32**> | The number of results per page. Only applies when using a date range |  |

### Return type

[**crate::models::CalendarDayCollection**](CalendarDayCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_timeslots

> crate::models::TimeslotCollection get_available_timeslots(filter_user_ids, filter_appointment_id, filter_start_at, filter_end_at, filter_timeframe, duration, interval, page, per_page)
Fetch available timeslots for a user or group

Fetch available timeslots for scheduling or rescheduling an appointment. Timeslots can be retrieved using a specific start & end date range, or using a timeframe. When using a timeframe, the page parameter can be used to flip through days or weeks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_user_ids** | Option<[**Vec<String>**](String.md)> | The IDs of users whose appointments will be retrieved. UUID Version 4. |  |
**filter_appointment_id** | Option<**String**> | Appointment ID used to fetch availability for an existing order |  |
**filter_start_at** | Option<**String**> | Returns availability after start_at |  |
**filter_end_at** | Option<**String**> | Returns availability before end_at |  |
**filter_timeframe** | Option<[**Vec<String>**](String.md)> | Returns availability for a specific timeframe. Used instead of start_at & end_at |  |
**duration** | Option<**i32**> | Duration of the event to schedule. Required if appointment_id isn't specified |  |
**interval** | Option<**i32**> | Interval of bookable timeslots starting at x minutes on the hour . Required if appointment_id isn't specified |  |
**page** | Option<**i32**> | The requested page of results |  |
**per_page** | Option<**i32**> | The number of results per page. Only applies when using a date range |  |

### Return type

[**crate::models::TimeslotCollection**](TimeslotCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_appointments

> crate::models::UnconfirmedAppointmentCollection get_unconfirmed_appointments(include, filter_user_ids, sort, per_page, page)
List all unconfirmed appointments.

List all unconfirmed appointments. These are appointments that have not been scheduled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |
**filter_user_ids** | Option<[**Vec<String>**](String.md)> | The IDs of users whose appointments will be retrieved. UUID Version 4. |  |
**sort** | Option<**String**> | Comma separated list of fields used for sorting. Placing a minus symbol in front of a field name sorts in descending order. Defaults to `-start_at`. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |

### Return type

[**crate::models::UnconfirmedAppointmentCollection**](UnconfirmedAppointmentCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_appointments_id

> crate::models::UnconfirmedAppointmentResource get_unconfirmed_appointments_id(unconfirmed_appointment_id, include)
Retrieve an unconfirmed appointment.

Retrieves the details of an unconfirmed appointment with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unconfirmed_appointment_id** | [**String**](.md) | The ID of an appointment. | [required] |
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |

### Return type

[**crate::models::UnconfirmedAppointmentResource**](UnconfirmedAppointmentResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_appointments_appointment_id_cancel

> crate::models::AppointmentResource put_appointments_appointment_id_cancel(appointment_id, appointment_cancel_put_payload)
Cancel an appointment.

Cancel an appointment. The appointments order's customer can be optionally notified of this change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | [**String**](.md) | The ID of an appointment. | [required] |
**appointment_cancel_put_payload** | Option<[**AppointmentCancelPutPayload**](AppointmentCancelPutPayload.md)> |  |  |

### Return type

[**crate::models::AppointmentResource**](AppointmentResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_appointments_appointment_id_reschedule

> crate::models::AppointmentResource put_appointments_appointment_id_reschedule(appointment_id, appointment_reschedule_put_payload)
Reschedule an appointment.

Reschedule an appointment. The appointments order's customer can be optionally notified of this change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | [**String**](.md) | The ID of an appointment. | [required] |
**appointment_reschedule_put_payload** | Option<[**AppointmentReschedulePutPayload**](AppointmentReschedulePutPayload.md)> |  |  |

### Return type

[**crate::models::AppointmentResource**](AppointmentResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

