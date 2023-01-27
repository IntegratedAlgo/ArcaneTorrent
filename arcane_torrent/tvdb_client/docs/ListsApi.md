# \ListsApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_lists**](ListsApi.md#get_all_lists) | **GET** /lists | 
[**get_list**](ListsApi.md#get_list) | **GET** /lists/{id} | 
[**get_list_by_slug**](ListsApi.md#get_list_by_slug) | **GET** /lists/slug/{slug} | 
[**get_list_extended**](ListsApi.md#get_list_extended) | **GET** /lists/{id}/extended | 
[**get_list_translation**](ListsApi.md#get_list_translation) | **GET** /lists/{id}/translations/{language} | 



## get_all_lists

> crate::models::GetAllLists200Response get_all_lists(page)


returns list of list base records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f32**> | page number |  |

### Return type

[**crate::models::GetAllLists200Response**](getAllLists_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list

> crate::models::GetList200Response get_list(id)


returns an list base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetList200Response**](getList_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_by_slug

> crate::models::GetList200Response get_list_by_slug(slug)


returns an list base record search by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | slug | [required] |

### Return type

[**crate::models::GetList200Response**](getList_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_extended

> crate::models::GetListExtended200Response get_list_extended(id)


returns a list extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetListExtended200Response**](getListExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_list_translation

> crate::models::GetListTranslation200Response get_list_translation(id, language)


Returns list translation record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |
**language** | **String** | language | [required] |

### Return type

[**crate::models::GetListTranslation200Response**](getListTranslation_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

