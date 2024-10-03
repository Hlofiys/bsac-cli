use serde::{Serialize, Deserialize};
use super::{DayOfWeek, Group, Lesson, Teacher};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cabinet: Option<i64>,
    #[serde(rename = "dayOfWeek")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(rename = "groupId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson: Option<Lesson>,
    #[serde(rename = "lessonId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_id: Option<i64>,
    #[serde(rename = "lessonNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_number: Option<i64>,
    #[serde(rename = "subGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<i64>,
    #[serde(rename = "subNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teacher: Option<Teacher>,
    #[serde(rename = "teacherId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teacher_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub week: Option<String>,
}
impl std::fmt::Display for LessonSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
