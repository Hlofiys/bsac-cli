use serde::{Serialize, Deserialize};
use super::{Group, Lesson, Teacher};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exam {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cabinet: Option<i64>,
    #[serde(rename = "examStart")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exam_start: Option<chrono::DateTime<chrono::Utc>>,
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
}
impl std::fmt::Display for Exam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
