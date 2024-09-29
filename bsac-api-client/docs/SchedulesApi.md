# \SchedulesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_schedule_actuality**](SchedulesApi.md#get_schedule_actuality) | **GET** /api/schedules/actuality | Gets the schedule actuality
[**get_schedule_for_date**](SchedulesApi.md#get_schedule_for_date) | **GET** /api/schedules/groups/{groupId}/date | Get schedule for date
[**get_schedule_for_groups**](SchedulesApi.md#get_schedule_for_groups) | **GET** /api/schedules/groups | Gets schedule for users subscribed groups
[**get_schedule_for_teachers**](SchedulesApi.md#get_schedule_for_teachers) | **GET** /api/schedules/teachers | Gets schedule for user subscribed teachers
[**get_week**](SchedulesApi.md#get_week) | **GET** /api/schedules/week | Gets current week
[**get_works_schedule**](SchedulesApi.md#get_works_schedule) | **GET** /api/schedules/works | Get works schedule



## get_schedule_actuality

> models::BooleanServiceResponse get_schedule_actuality(last_interact)
Gets the schedule actuality

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last_interact** | Option<**String**> |  |  |

### Return type

[**models::BooleanServiceResponse**](BooleanServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_for_date

> models::GetScheduleForOneGroupListServiceResponse get_schedule_for_date(group_id, dates)
Get schedule for date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** |  | [required] |
**dates** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::GetScheduleForOneGroupListServiceResponse**](GetScheduleForOneGroupListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_for_groups

> models::GetScheduleForListOfGroupsListServiceResponse get_schedule_for_groups(groups_ids)
Gets schedule for users subscribed groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groups_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::GetScheduleForListOfGroupsListServiceResponse**](GetScheduleForListOfGroupsListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_for_teachers

> models::GetScheduleForListOfGroupsListServiceResponse get_schedule_for_teachers(teachers_ids)
Gets schedule for user subscribed teachers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**teachers_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::GetScheduleForListOfGroupsListServiceResponse**](GetScheduleForListOfGroupsListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_week

> models::Int32ServiceResponse get_week()
Gets current week

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Int32ServiceResponse**](Int32ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_works_schedule

> models::GetWorksScheduleDtoListServiceResponse get_works_schedule(teacher_id, group_id, lesson_id)
Get works schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**teacher_id** | Option<**i32**> |  |  |
**group_id** | Option<**i32**> |  |  |
**lesson_id** | Option<**i32**> |  |  |

### Return type

[**models::GetWorksScheduleDtoListServiceResponse**](GetWorksScheduleDtoListServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

