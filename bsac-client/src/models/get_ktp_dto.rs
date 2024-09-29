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
pub struct GetKtpDto {
    #[serde(rename = "ktp", skip_serializing_if = "Option::is_none")]
    pub ktp: Option<Box<models::Ktp>>,
    #[serde(rename = "works", skip_serializing_if = "Option::is_none")]
    pub works: Option<Box<models::GetKtpWorksDto>>,
}

impl GetKtpDto {
    pub fn new() -> GetKtpDto {
        GetKtpDto {
            ktp: None,
            works: None,
        }
    }
}

