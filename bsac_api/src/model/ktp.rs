use serde::{Serialize, Deserialize};
use super::{EducationTypes, Lesson};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ktp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course: Option<i64>,
    #[serde(rename = "educationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_type: Option<EducationTypes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson: Option<Lesson>,
    #[serde(rename = "lessonHours")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_hours: Option<i64>,
    #[serde(rename = "lessonId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_id: Option<i64>,
}
impl std::fmt::Display for Ktp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
