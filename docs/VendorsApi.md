# \VendorsApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_vendors**](VendorsApi.md#get_vendors) | **GET** /vendors | List all vendors.
[**get_vendors_id**](VendorsApi.md#get_vendors_id) | **GET** /vendors/{vendor_id} | Retrieve a vendor.



## get_vendors

> crate::models::GroupCollection get_vendors(include)
List all vendors.

Lists all vendors available to a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |

### Return type

[**crate::models::GroupCollection**](GroupCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vendors_id

> crate::models::GroupResource get_vendors_id(vendor_id, include)
Retrieve a vendor.

Retrieves the details of a vendor with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vendor_id** | [**String**](.md) | The ID of the group that is associated as a vendor. UUID Version 4. | [required] |
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |

### Return type

[**crate::models::GroupResource**](GroupResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

