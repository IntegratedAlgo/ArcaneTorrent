# \UpdatesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**updates**](UpdatesApi.md#updates) | **GET** /updates | 



## updates

> crate::models::Updates200Response updates(since, r#type, action, page)


Returns updated entities.  methodInt indicates a created record (1), an updated record (2), or a deleted record (3).  If a record is deleted because it was a duplicate of another record, the target record's information is provided in mergeToType and mergeToId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | **f32** |  | [required] |
**r#type** | Option<**String**> |  |  |
**action** | Option<**String**> |  |  |
**page** | Option<**f32**> | name |  |

### Return type

[**crate::models::Updates200Response**](updates_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

