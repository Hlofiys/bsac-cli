use serde::{Serialize, Deserialize};
use super::{Group, Lesson, Teacher};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cabinet: Option<i64>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teacher: Option<Teacher>,
    #[serde(rename = "teacherId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teacher_id: Option<i64>,
    #[serde(rename = "toDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_date: Option<chrono::NaiveDate>,
    #[serde(rename = "toLessonNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_lesson_number: Option<i64>,
    #[serde(rename = "toSubGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_sub_group: Option<i64>,
    #[serde(rename = "toSubLessonNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_sub_lesson_number: Option<i64>,
}
impl std::fmt::Display for ScheduleAdd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
