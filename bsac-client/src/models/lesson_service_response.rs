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
pub struct LessonServiceResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::Lesson>>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "responseCode", skip_serializing_if = "Option::is_none")]
    pub response_code: Option<models::HttpStatusCode>,
}

impl LessonServiceResponse {
    pub fn new() -> LessonServiceResponse {
        LessonServiceResponse {
            data: None,
            success: None,
            message: None,
            response_code: None,
        }
    }
}

