# \ListingsApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_listings**](ListingsApi.md#get_listings) | **GET** /listings | Get the listings available to a group.
[**get_listings_id**](ListingsApi.md#get_listings_id) | **GET** /listings/{uuid} | Get information about a listing.



## get_listings

> crate::models::PartialListingCollection get_listings(query, per_page, page, status, price, bathrooms, bedrooms)
Get the listings available to a group.

Get the listings available to a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | A search query. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |
**status** | Option<**String**> | The status you want to scope down to, example sold,  coming_soon,  for_sale, sold |  |
**price** | Option<**i32**> | The price value and greater will be returned. |  |
**bathrooms** | Option<**f32**> | Number of bathrooms minimum. |  |
**bedrooms** | Option<**i32**> | Number of bedrooms minimum. |  |

### Return type

[**crate::models::PartialListingCollection**](PartialListingCollection.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listings_id

> crate::models::ListingResource get_listings_id(uuid)
Get information about a listing.

Get information about a listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | [**String**](.md) | The UUID of a listing. | [required] |

### Return type

[**crate::models::ListingResource**](ListingResource.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

