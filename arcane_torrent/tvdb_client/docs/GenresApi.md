# \GenresApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_genres**](GenresApi.md#get_all_genres) | **GET** /genres | 
[**get_genre_base**](GenresApi.md#get_genre_base) | **GET** /genres/{id} | 



## get_all_genres

> crate::models::GetAllGenres200Response get_all_genres()


returns list of genre records

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAllGenres200Response**](getAllGenres_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_genre_base

> crate::models::GetGenreBase200Response get_genre_base(id)


Returns genre record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetGenreBase200Response**](getGenreBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

