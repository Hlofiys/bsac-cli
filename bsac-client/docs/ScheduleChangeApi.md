# \ScheduleChangeApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_schedule_additions**](ScheduleChangeApi.md#get_schedule_additions) | **GET** /api/schedulechange/additions/{groupId} | Gets all schedule additions for group
[**get_schedule_moves**](ScheduleChangeApi.md#get_schedule_moves) | **GET** /api/schedulechange/move/{groupId} | Gets all schedule moves for group



## get_schedule_additions

> models::ScheduleAddListServiceResponse get_schedule_additions(group_id)
Gets all schedule additions for group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** |  | [required] |

### Return type

[**models::ScheduleAddListServiceResponse**](ScheduleAddListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_moves

> models::ScheduleAddListServiceResponse get_schedule_moves(group_id)
Gets all schedule moves for group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** |  | [required] |

### Return type

[**models::ScheduleAddListServiceResponse**](ScheduleAddListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

