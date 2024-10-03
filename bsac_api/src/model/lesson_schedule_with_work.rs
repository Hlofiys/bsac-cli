use serde::{Serialize, Deserialize};
use super::{LessonSchedule, ScheduleAdd, ScheduleMove, Work};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonScheduleWithWork {
    #[serde(rename = "lessonSchedule")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_schedule: Option<LessonSchedule>,
    #[serde(rename = "scheduleAdd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_add: Option<ScheduleAdd>,
    #[serde(rename = "scheduleMove")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_move: Option<ScheduleMove>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work: Option<Work>,
}
impl std::fmt::Display for LessonScheduleWithWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
