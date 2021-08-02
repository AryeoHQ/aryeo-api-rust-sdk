# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | ID of the group. | 
**_type** | **String** | The type of the group. Can be CREATOR, AGENT, or BROKERAGE, and may dictate the attributes of the group returned. | 
**name** | **String** | The name of the group. | 
**email** | Option<**String**> | The email address of a group. | [optional]
**phone** | Option<**String**> | A phone number represented in whichever standards specified by the group, typically ###-###-#### (separated by hyphens). | [optional]
**website_url** | Option<**String**> | The website URL of a group. | [optional]
**logo_url** | Option<**String**> | The logo URL of a group. | [optional]
**avatar_url** | Option<**String**> | The profile image URL of a real estate agent. Only returned if group's type is AGENT. | [optional]
**office_name** | Option<**String**> | The name of the brokerage or team of a real estate agent. Only returned if group's type is AGENT. | [optional]
**license_number** | Option<**String**> | The license number of a real estate agent. Only returned if group's type is AGENT. | [optional]
**social_profiles** | Option<[**crate::models::SocialProfiles**](SocialProfiles.md)> |  | [optional]
**default_order_form** | Option<[**crate::models::OrderForm**](OrderForm.md)> |  | [optional]
**order_forms** | Option<[**Vec<crate::models::OrderForm>**](OrderForm.md)> | An array of order forms a vendor group provides for placing orders. Only returned if group's type is CREATOR.  | [optional]
**owner** | Option<[**crate::models::User**](User.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::User>**](User.md)> | The Aryeo users associated with this group. | [optional]
**is_brokerage_or_brokerage_agent** | **bool** | Does this group represent a brokerage or an agent who belongs to a brokerage? | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


