# \VendorsApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_vendors**](VendorsApi.md#get_vendors) | **GET** /vendors | Get vendors available to a group.
[**get_vendors_search**](VendorsApi.md#get_vendors_search) | **GET** /vendors/search | Get vendors that can be added to the group's vendor list.



## get_vendors

> crate::models::GroupCollection get_vendors()
Get vendors available to a group.

Get vendors available to a group.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GroupCollection**](GroupCollection.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vendors_search

> crate::models::GroupCollection get_vendors_search(query, per_page, page)
Get vendors that can be added to the group's vendor list.

Get vendors that can be added to the group's vendor list, excluding those already available to a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | A search query. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |

### Return type

[**crate::models::GroupCollection**](GroupCollection.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

