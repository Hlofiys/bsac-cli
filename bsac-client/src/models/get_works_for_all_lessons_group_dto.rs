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
pub struct GetWorksForAllLessonsGroupDto {
    #[serde(rename = "worksList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Option<Vec<models::GetWorksGroupDto>>>,
    #[serde(rename = "lesson", skip_serializing_if = "Option::is_none")]
    pub lesson: Option<Box<models::Lesson>>,
    #[serde(rename = "calculated", skip_serializing_if = "Option::is_none")]
    pub calculated: Option<bool>,
}

impl GetWorksForAllLessonsGroupDto {
    pub fn new() -> GetWorksForAllLessonsGroupDto {
        GetWorksForAllLessonsGroupDto {
            works_list: None,
            lesson: None,
            calculated: None,
        }
    }
}

