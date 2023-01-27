# \LoginApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login_post**](LoginApi.md#login_post) | **POST** /login | create an auth token. The token has one month valition length.



## login_post

> crate::models::LoginPost200Response login_post(login_post_request)
create an auth token. The token has one month valition length.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_post_request** | [**LoginPostRequest**](LoginPostRequest.md) |  | [required] |

### Return type

[**crate::models::LoginPost200Response**](_login_post_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

