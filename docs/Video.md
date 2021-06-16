# Video

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | ID of the video. | 
**title** | Option<**String**> | The title of the video given by the uploading user. | [optional]
**display_type** | **String** | The display type determines if the video is branded or unbranded (can also be none or both). This affects whether the video is displayed on branded or unbranded marketing materials such as the property website. | 
**source_type** | **String** | The original upload source of the video, used to determine how to handle the playback_url of the video and other display properties.  | 
**thumbnail_url** | **String** | Thumbnail URL for the video. | 
**playback_url** | **String** | A URL linking to the video. | 
**download_url** | Option<**String**> | A URL for downloading the video. | [optional]
**seconds** | Option<**i32**> | The video's runtime in seconds. | [optional]
**share_url** | Option<**String**> | Aryeo iFrame player URL | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


