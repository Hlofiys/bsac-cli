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
pub struct User {
    #[serde(rename = "deviceId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Option<String>>,
    #[serde(rename = "groupId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<i32>>,
    #[serde(rename = "registerDate", skip_serializing_if = "Option::is_none")]
    pub register_date: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            device_id: None,
            group_id: None,
            register_date: None,
        }
    }
}

