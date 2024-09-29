# \ExamsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_group_exams**](ExamsApi.md#get_group_exams) | **GET** /api/exams/{groupId} | Gets all exams for group



## get_group_exams

> models::ExamListServiceResponse get_group_exams(group_id)
Gets all exams for group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** |  | [required] |

### Return type

[**models::ExamListServiceResponse**](ExamListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

