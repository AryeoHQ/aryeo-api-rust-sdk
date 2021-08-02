# \ListingsApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_listings**](ListingsApi.md#get_listings) | **GET** /listings | Get the listings available to a group.
[**get_listings_id**](ListingsApi.md#get_listings_id) | **GET** /listings/{listing_id} | Get information about a listing.



## get_listings

> crate::models::ListingCollection get_listings(include, filter_search, filter_address, filter_list_agent, filter_status, filter_active, filter_price_gte, filter_price_lte, filter_square_feet_gte, filter_square_feet_lte, filter_bedrooms_gte, filter_bedrooms_lte, filter_bathrooms_gte, filter_bathrooms_lte, sort, per_page, page)
Get the listings available to a group.

Get the listings available to a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |
**filter_search** | Option<**String**> | Return listings that have fields matching this term. |  |
**filter_address** | Option<**String**> | Return listings that have an address matching this term. |  |
**filter_list_agent** | Option<**String**> | Return listings that have a listing agent or co-listing agent matching this term. |  |
**filter_status** | Option<**String**> | Return listings that have a certain status. |  |
**filter_active** | Option<**bool**> | Set as true to return listings that have an active status (e.g. active statuses include `COMING_SOON`, `FOR_SALE`, `FOR_LEASE`, `PENDING_SALE`, `PENDING_LEASE`, `SOLD`, `LEASED`).  |  |
**filter_price_gte** | Option<**f32**> | Return listings where the price field is greater than or equal to this value. |  |
**filter_price_lte** | Option<**f32**> | Return listings where the price field is less than or equal to this value. |  |
**filter_square_feet_gte** | Option<**f32**> | Return listings where the square feet field is greater than or equal to this value. |  |
**filter_square_feet_lte** | Option<**f32**> | Return listings where the square feet field is less than or equal to this value. |  |
**filter_bedrooms_gte** | Option<**i32**> | Return listings where the bedrooms field is greater than or equal to this value. |  |
**filter_bedrooms_lte** | Option<**i32**> | Return listings where the bedrooms field is less than or equal to this value. |  |
**filter_bathrooms_gte** | Option<**f32**> | Return listings where the bathrooms field is greater than or equal to this value. |  |
**filter_bathrooms_lte** | Option<**f32**> | Return listings where the bathrooms field is less than or equal to this value. |  |
**sort** | Option<**String**> | Comma separated list of fields used for sorting. Placing a minus symbol in front of a field name sorts in descending order. Defaults to `-created_at`. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |

### Return type

[**crate::models::ListingCollection**](ListingCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listings_id

> crate::models::ListingResource get_listings_id(listing_id, include)
Get information about a listing.

Get information about a listing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listing_id** | [**String**](.md) | The ID of a listing. | [required] |
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |

### Return type

[**crate::models::ListingResource**](ListingResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

