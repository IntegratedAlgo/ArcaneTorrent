# \UserInfoApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_info**](UserInfoApi.md#get_user_info) | **GET** /user | 
[**get_user_info_by_id**](UserInfoApi.md#get_user_info_by_id) | **GET** /user/{id} | 



## get_user_info

> crate::models::GetUserInfo200Response get_user_info()


returns user info

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetUserInfo200Response**](getUserInfo_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_info_by_id

> crate::models::GetUserInfo200Response get_user_info_by_id(id)


returns user info by user id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetUserInfo200Response**](getUserInfo_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

