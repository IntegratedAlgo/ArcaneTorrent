# \CompaniesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_companies**](CompaniesApi.md#get_all_companies) | **GET** /companies | 
[**get_company**](CompaniesApi.md#get_company) | **GET** /companies/{id} | 
[**get_company_types**](CompaniesApi.md#get_company_types) | **GET** /companies/types | 



## get_all_companies

> crate::models::GetAllCompanies200Response get_all_companies(page)


returns a paginated list of company records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f32**> | name |  |

### Return type

[**crate::models::GetAllCompanies200Response**](getAllCompanies_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company

> crate::models::GetCompany200Response get_company(id)


returns a company record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f32** | id | [required] |

### Return type

[**crate::models::GetCompany200Response**](getCompany_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_company_types

> crate::models::GetCompanyTypes200Response get_company_types()


returns all company type records

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCompanyTypes200Response**](getCompanyTypes_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

