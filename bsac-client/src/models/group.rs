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
pub struct Group {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "groupNumber", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_number: Option<Option<String>>,
    #[serde(rename = "course", skip_serializing_if = "Option::is_none")]
    pub course: Option<i32>,
    #[serde(rename = "educationType", skip_serializing_if = "Option::is_none")]
    pub education_type: Option<models::EducationTypes>,
    #[serde(rename = "removable", skip_serializing_if = "Option::is_none")]
    pub removable: Option<bool>,
    #[serde(rename = "semesterStart", skip_serializing_if = "Option::is_none")]
    pub semester_start: Option<String>,
    #[serde(rename = "semesterEnd", skip_serializing_if = "Option::is_none")]
    pub semester_end: Option<String>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            id: None,
            group_number: None,
            course: None,
            education_type: None,
            removable: None,
            semester_start: None,
            semester_end: None,
        }
    }
}

