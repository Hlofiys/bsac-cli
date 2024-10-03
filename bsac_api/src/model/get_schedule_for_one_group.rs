use serde::{Serialize, Deserialize};
use super::{Exam, LessonScheduleWithWork, Practice};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleForOneGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exam: Option<Vec<Exam>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub practice: Option<Practice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<LessonScheduleWithWork>>,
}
impl std::fmt::Display for GetScheduleForOneGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
