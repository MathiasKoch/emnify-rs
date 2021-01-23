# \LookupsApi

All URIs are relative to *https://cdn.emnify.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**breakout_region_get**](LookupsApi.md#breakout_region_get) | **get** /api/v1/breakout_region | List Breakout Regions
[**country_get**](LookupsApi.md#country_get) | **get** /api/v1/country | List Country Codes
[**currency_get**](LookupsApi.md#currency_get) | **get** /api/v1/currency | List Currencies
[**data_blocksize_get**](LookupsApi.md#data_blocksize_get) | **get** /api/v1/data_blocksize | List Data blocksizes
[**data_throttle_get**](LookupsApi.md#data_throttle_get) | **get** /api/v1/data_throttle | List Data Throttles
[**esme_interface_type_get**](LookupsApi.md#esme_interface_type_get) | **get** /api/v1/esme_interface_type | List ESME Interface Types
[**rat_type**](LookupsApi.md#rat_type) | **get** /api/v1/rat_type | List RAT types



## breakout_region_get

> Vec<crate::models::RetrieveAvailableBreakoutRegionsresponse> breakout_region_get()
List Breakout Regions

Provides the list of available breakout regions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableBreakoutRegionsresponse>**](RetrieveAvailableBreakoutRegionsresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## country_get

> Vec<crate::models::RetrieveAvailableCountriesresponse> country_get()
List Country Codes

Provides the list of available countries (lookup).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableCountriesresponse>**](RetrieveAvailableCountriesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## currency_get

> Vec<crate::models::RetrieveAvailableCurrenciesresponse> currency_get()
List Currencies

Provides the list of available currencies (lookup).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableCurrenciesresponse>**](RetrieveAvailableCurrenciesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_blocksize_get

> Vec<crate::models::RetrieveAvailableDataBlocksizesresponse> data_blocksize_get()
List Data blocksizes

Provides the list of available data blocksizes (lookup).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableDataBlocksizesresponse>**](RetrieveAvailableDataBlocksizesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_throttle_get

> Vec<crate::models::RetrieveAvailableDataThrottlesresponse> data_throttle_get()
List Data Throttles

Provides the list of available data throttles (lookup).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableDataThrottlesresponse>**](RetrieveAvailableDataThrottlesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## esme_interface_type_get

> Vec<crate::models::RetrieveAvailableEsmeInterfaceTypesresponse> esme_interface_type_get()
List ESME Interface Types

Provides the list of available ESME interface types.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RetrieveAvailableEsmeInterfaceTypesresponse>**](RetrieveAvailableESMEInterfaceTypesresponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rat_type

> Vec<crate::models::RatType> rat_type(blacklistable, limitable)
List RAT types

Returns a list of supported RAT types. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blacklistable** | Option<**bool**> | Returns a list of RAT types that can be blacklisted for network coverage data.  `/api/v1/rat_type?blacklistable`  |  |
**limitable** | Option<**bool**> | Returns a list of RAT types that can be limited for a pdp_context_definition  `/api/v1/rat_type?limitable`  |  |

### Return type

[**Vec<crate::models::RatType>**](RAT_Type.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

