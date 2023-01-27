# \SearchApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search_results**](SearchApi.md#get_search_results) | **GET** /search | 
[**get_search_results_by_remote_id**](SearchApi.md#get_search_results_by_remote_id) | **GET** /search/remoteid/{remoteId} | 



## get_search_results

> crate::models::GetSearchResults200Response get_search_results(query, q, r#type, year, company, country, director, language, primary_type, network, remote_id, offset, limit)


Our search index includes series, movies, people, and companies. Search is limited to 5k results max.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | The primary search string, which can include the main title for a record including all translations and aliases. |  |
**q** | Option<**String**> | Alias of the \"query\" parameter.  Recommend using query instead as this field will eventually be deprecated. |  |
**r#type** | Option<**String**> | Restrict results to a specific entity type.  Can be movie, series, person, or company. |  |
**year** | Option<**f32**> | Restrict results to a specific year. Currently only used for series and movies. |  |
**company** | Option<**String**> | Restrict results to a specific company (original network, production company, studio, etc).  As an example, \"The Walking Dead\" would have companies of \"AMC\", \"AMC+\", and \"Disney+\". |  |
**country** | Option<**String**> | Restrict results to a specific country of origin. Should contain a 3 character country code. Currently only used for series and movies. |  |
**director** | Option<**String**> | Restrict results to a specific director.  Generally only used for movies.  Should include the full name of the director, such as \"Steven Spielberg\". |  |
**language** | Option<**String**> | Restrict results to a specific primary language.  Should include the 3 character language code.  Currently only used for series and movies. |  |
**primary_type** | Option<**String**> | Restrict results to a specific type of company.  Should include the full name of the type of company, such as \"Production Company\".  Only used for companies. |  |
**network** | Option<**String**> | Restrict results to a specific network.  Used for TV and TV movies, and functions the same as the company parameter with more specificity. |  |
**remote_id** | Option<**String**> | Search for a specific remote id.  Allows searching for an IMDB or EIDR id, for example. |  |
**offset** | Option<**f32**> | Offset results. |  |
**limit** | Option<**f32**> | Limit results. |  |

### Return type

[**crate::models::GetSearchResults200Response**](getSearchResults_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_results_by_remote_id

> crate::models::GetSearchResultsByRemoteId200Response get_search_results_by_remote_id(remote_id)


Search a series, movie, people, episode, company or season by specific remote id and returns a base record for that entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_id** | **String** | Search for a specific remote id.  Allows searching for an IMDB or EIDR id, for example. | [required] |

### Return type

[**crate::models::GetSearchResultsByRemoteId200Response**](getSearchResultsByRemoteId_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

