use serde::{Serialize, Deserialize};
use super::{HttpStatusCode, ScheduleAdd};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAddListServiceResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ScheduleAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_code: Option<HttpStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
impl std::fmt::Display for ScheduleAddListServiceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
