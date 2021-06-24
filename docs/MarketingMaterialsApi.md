# \MarketingMaterialsApi

All URIs are relative to *https://api.aryeo.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**put_marketing_materials_templates_uuid_publish**](MarketingMaterialsApi.md#put_marketing_materials_templates_uuid_publish) | **PUT** /marketing-materials/templates/{uuid}/publish | Publish a marketing material template.
[**put_marketing_materials_uuid_publish**](MarketingMaterialsApi.md#put_marketing_materials_uuid_publish) | **PUT** /marketing-materials/{uuid}/publish | Publish a marketing material.



## put_marketing_materials_templates_uuid_publish

> put_marketing_materials_templates_uuid_publish(uuid, marketing_material_template_publish_payload)
Publish a marketing material template.

Publish a marketing material template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | [**String**](.md) | UUID of the marketing material template record. | [required] |
**marketing_material_template_publish_payload** | Option<[**MarketingMaterialTemplatePublishPayload**](MarketingMaterialTemplatePublishPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_marketing_materials_uuid_publish

> put_marketing_materials_uuid_publish(uuid, marketing_material_publish_payload)
Publish a marketing material.

Publish a marketing material.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | [**String**](.md) | UUID of the marketing material record. | [required] |
**marketing_material_publish_payload** | Option<[**MarketingMaterialPublishPayload**](MarketingMaterialPublishPayload.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[JWT](../README.md#JWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

