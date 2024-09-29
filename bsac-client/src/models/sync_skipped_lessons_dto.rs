/*
 * BsacScheduleApp
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncSkippedLessonsDto {
    #[serde(rename = "lessonScheduleId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lesson_schedule_id: Option<Option<i32>>,
    #[serde(rename = "practiceId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<Option<i32>>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl SyncSkippedLessonsDto {
    pub fn new() -> SyncSkippedLessonsDto {
        SyncSkippedLessonsDto {
            lesson_schedule_id: None,
            practice_id: None,
            date: None,
        }
    }
}

