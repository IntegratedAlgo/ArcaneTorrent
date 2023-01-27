# \FavoritesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_favorites**](FavoritesApi.md#create_user_favorites) | **POST** /user/favorites | 
[**get_user_favorites**](FavoritesApi.md#get_user_favorites) | **GET** /user/favorites | 



## create_user_favorites

> create_user_favorites(favorite_record)


creates a new user favorite

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**favorite_record** | Option<[**FavoriteRecord**](FavoriteRecord.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_favorites

> crate::models::GetUserFavorites200Response get_user_favorites()


returns user favorites

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetUserFavorites200Response**](getUserFavorites_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

