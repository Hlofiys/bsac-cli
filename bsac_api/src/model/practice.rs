use serde::{Serialize, Deserialize};
use super::{Group, Teacher};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cabinet: Option<i64>,
    #[serde(rename = "endDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(rename = "endTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(rename = "groupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "startDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(rename = "startTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teacher: Option<Teacher>,
    #[serde(rename = "teacherId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teacher_id: Option<i64>,
}
impl std::fmt::Display for Practice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
