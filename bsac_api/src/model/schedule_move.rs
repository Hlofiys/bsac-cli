use serde::{Serialize, Deserialize};
use super::LessonSchedule;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleMove {
    #[serde(rename = "fromDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_date: Option<chrono::NaiveDate>,
    #[serde(rename = "fromLessonSchedule")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_lesson_schedule: Option<LessonSchedule>,
    #[serde(rename = "fromLessonScheduleId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_lesson_schedule_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "toDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<chrono::NaiveDate>,
    #[serde(rename = "toLessonNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_lesson_number: Option<i64>,
    #[serde(rename = "toSubLessonNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_sub_lesson_number: Option<i64>,
}
impl std::fmt::Display for ScheduleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
