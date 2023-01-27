# \SeriesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_series**](SeriesApi.md#get_all_series) | **GET** /series | 
[**get_series_artworks**](SeriesApi.md#get_series_artworks) | **GET** /series/{id}/artworks | 
[**get_series_base**](SeriesApi.md#get_series_base) | **GET** /series/{id} | 
[**get_series_base_by_slug**](SeriesApi.md#get_series_base_by_slug) | **GET** /series/slug/{slug} | 
[**get_series_episodes**](SeriesApi.md#get_series_episodes) | **GET** /series/{id}/episodes/{season-type} | 
[**get_series_extended**](SeriesApi.md#get_series_extended) | **GET** /series/{id}/extended | 
[**get_series_filter**](SeriesApi.md#get_series_filter) | **GET** /series/filter | 
[**get_series_next_aired**](SeriesApi.md#get_series_next_aired) | **GET** /series/{id}/nextAired | 
[**get_series_season_episodes_translated**](SeriesApi.md#get_series_season_episodes_translated) | **GET** /series/{id}/episodes/{season-type}/{lang} | 
[**get_series_translation**](SeriesApi.md#get_series_translation) | **GET** /series/{id}/translations/{language} | 



## get_all_series

> crate::models::GetAllSeries200Response get_all_series(page)


returns list of series base records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f32**> | page number |  |

### Return type

[**crate::models::GetAllSeries200Response**](getAllSeries_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_artworks

> crate::models::GetSeriesArtworks200Response get_series_artworks(id, lang, r#type)


Returns series artworks base on language and type. <br> Note&#58; Artwork type is an id that can be found using **_/artwork/types** endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |
**lang** | Option<**String**> | lang |  |
**r#type** | Option<**i32**> | type |  |

### Return type

[**crate::models::GetSeriesArtworks200Response**](getSeriesArtworks_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_base

> crate::models::GetSeriesBase200Response get_series_base(id)


Returns series base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetSeriesBase200Response**](getSeriesBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_base_by_slug

> crate::models::GetSeriesBase200Response get_series_base_by_slug(slug)


Returns series base record searched by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | slug | [required] |

### Return type

[**crate::models::GetSeriesBase200Response**](getSeriesBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_episodes

> crate::models::GetSeriesEpisodes200Response get_series_episodes(page, id, season_type, season, episode_number, air_date)


Returns series episodes from the specified season type, default returns the episodes in the series default season type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i32** |  | [required] |[default to 0]
**id** | **f32** | id | [required] |
**season_type** | **String** | season-type | [required] |
**season** | Option<**i32**> |  |  |[default to 0]
**episode_number** | Option<**i32**> |  |  |[default to 0]
**air_date** | Option<**String**> | airDate of the episode, format is yyyy-mm-dd |  |

### Return type

[**crate::models::GetSeriesEpisodes200Response**](getSeriesEpisodes_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_extended

> crate::models::GetSeriesArtworks200Response get_series_extended(id, meta, short)


Returns series extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |
**meta** | Option<**String**> | meta |  |
**short** | Option<**bool**> | reduce the payload and returns the short version of this record without characters and artworks |  |

### Return type

[**crate::models::GetSeriesArtworks200Response**](getSeriesArtworks_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_filter

> crate::models::GetSeriesFilter200Response get_series_filter(country, lang, company, content_rating, genre, sort, sort_type, status, year)


Search series based on filter parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | **String** | country of origin | [required] |
**lang** | **String** | original language | [required] |
**company** | Option<**f32**> | production company |  |
**content_rating** | Option<**f32**> | content rating id base on a country |  |
**genre** | Option<**f32**> | Genre id. This id can be found using **_/genres** endpoint. |  |
**sort** | Option<**String**> | sort by results |  |
**sort_type** | Option<**String**> | sort type ascending or descending |  |
**status** | Option<**f32**> | status |  |
**year** | Option<**f32**> | release year |  |

### Return type

[**crate::models::GetSeriesFilter200Response**](getSeriesFilter_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_next_aired

> crate::models::GetSeriesBase200Response get_series_next_aired(id)


Returns series base record including the nextAired field. <br> Note&#58; nextAired was included in the base record endpoint but that field will deprecated in the future so developers should use the nextAired endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetSeriesBase200Response**](getSeriesBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_season_episodes_translated

> crate::models::GetSeriesSeasonEpisodesTranslated200Response get_series_season_episodes_translated(page, id, season_type, lang)


Returns series base record with episodes from the specified season type and language. Default returns the episodes in the series default season type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | **i32** |  | [required] |[default to 0]
**id** | **f32** | id | [required] |
**season_type** | **String** | season-type | [required] |
**lang** | **String** |  | [required] |

### Return type

[**crate::models::GetSeriesSeasonEpisodesTranslated200Response**](getSeriesSeasonEpisodesTranslated_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_translation

> crate::models::GetEpisodeTranslation200Response get_series_translation(id, language)


Returns series translation record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |
**language** | **String** | language | [required] |

### Return type

[**crate::models::GetEpisodeTranslation200Response**](getEpisodeTranslation_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

