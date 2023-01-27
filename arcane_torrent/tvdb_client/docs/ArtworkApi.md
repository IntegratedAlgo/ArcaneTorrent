# \ArtworkApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_artwork_base**](ArtworkApi.md#get_artwork_base) | **GET** /artwork/{id} | 
[**get_artwork_extended**](ArtworkApi.md#get_artwork_extended) | **GET** /artwork/{id}/extended | 



## get_artwork_base

> crate::models::GetArtworkBase200Response get_artwork_base(id)


Returns a single artwork base record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetArtworkBase200Response**](getArtworkBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artwork_extended

> crate::models::GetArtworkExtended200Response get_artwork_extended(id)


Returns a single artwork extended record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetArtworkExtended200Response**](getArtworkExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

