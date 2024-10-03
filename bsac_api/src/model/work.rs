use serde::{Serialize, Deserialize};
use super::WorkTypeEnum;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::NaiveDate>,
    #[serde(rename = "groupKtpId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ktp_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lessonNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_number: Option<i64>,
    #[serde(rename = "lessonScheduleId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_schedule_id: Option<i64>,
    #[serde(rename = "workType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_type: Option<WorkTypeEnum>,
}
impl std::fmt::Display for Work {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
