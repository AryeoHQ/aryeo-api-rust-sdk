# \OrdersApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_orders**](OrdersApi.md#get_orders) | **GET** /orders | Get orders available to a group.
[**post_orders**](OrdersApi.md#post_orders) | **POST** /orders | Create an order.



## get_orders

> crate::models::OrderCollection get_orders()
Get orders available to a group.

Get orders of a group.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OrderCollection**](OrderCollection.md)

### Authorization

[JWT](../README.md#JWT)

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
**order_post_payload** | Option<[**OrderPostPayload**](OrderPostPayload.md)> |  |  |

### Return type

[**crate::models::OrderResource**](OrderResource.md)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

