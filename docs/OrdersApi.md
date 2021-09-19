# \OrdersApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_orders**](OrdersApi.md#get_orders) | **GET** /orders | List all orders.
[**get_orders_id**](OrdersApi.md#get_orders_id) | **GET** /orders/{order_id} | Retrieve an order.
[**get_products**](OrdersApi.md#get_products) | **GET** /products | List all products.
[**post_orders**](OrdersApi.md#post_orders) | **POST** /orders | Create an order.



## get_orders

> crate::models::OrderCollection get_orders(sort, per_page, page)
List all orders.

Lists all orders of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Comma separated list of fields used for sorting. Placing a minus symbol in front of a field name sorts in descending order. Defaults to `-created_at`. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |

### Return type

[**crate::models::OrderCollection**](OrderCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders_id

> crate::models::OrderResource get_orders_id(order_id, include)
Retrieve an order.

Retrieves the details of an order with the given ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | [**String**](.md) | The ID of an order. UUID Version 4. | [required] |
**include** | Option<**String**> | Comma separated list of optional data to include in the response. |  |

### Return type

[**crate::models::OrderResource**](OrderResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_products

> crate::models::ProductCollection get_products(sort, per_page, page, filter_search, filter_category_ids, filter_type)
List all products.

List all products of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Comma separated list of fields used for sorting. Placing a minus symbol in front of a field name sorts in descending order. Defaults to `title`. |  |
**per_page** | Option<**String**> | The number of items per page. Defaults to 25. |  |
**page** | Option<**String**> | The requested page. Defaults to 1. |  |
**filter_search** | Option<**String**> | Return products that have fields matching this term. |  |
**filter_category_ids** | Option<[**Vec<String>**](String.md)> | Return products in the given categories. |  |
**filter_type** | Option<**String**> | Return products matching the given type. Allowed values are: MAIN, ADDON. |  |

### Return type

[**crate::models::ProductCollection**](ProductCollection.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orders

> crate::models::OrderResource post_orders(order_post_payload)
Create an order.

Create an order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_post_payload** | Option<[**OrderPostPayload**](OrderPostPayload.md)> | OrderPostPayload |  |

### Return type

[**crate::models::OrderResource**](OrderResource.md)

### Authorization

[Token](../README.md#Token)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

