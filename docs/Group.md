# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the group. | 
**group_type** | **String** | The type of group. | 
**name** | **String** | The name of the group. | 
**logo** | Option<**String**> | Group logo. | [optional]
**email** | Option<**String**> | Email. | [optional]
**phone** | Option<**String**> | Phone number. | [optional]
**website** | Option<**String**> | Website. | [optional]
**is_brokerage_or_brokerage_agent** | **bool** | Does this group represent a brokerage or an agent who belongs to a brokerage? | 
**social_profiles** | Option<[**crate::models::SocialProfiles**](SocialProfiles.md)> |  | [optional]
**agent_properties** | Option<[**crate::models::GroupAgentProperties**](GroupAgentProperties.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::User>**](User.md)> | users | [optional]
**default_order_form** | Option<[**crate::models::OrderForm**](OrderForm.md)> |  | [optional]
**order_forms** | Option<[**Vec<crate::models::OrderForm>**](OrderForm.md)> | An array of order forms. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


