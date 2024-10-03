use serde::{Serialize, Deserialize};
use super::WorkTypeEnum;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorksGroupDto {
    #[serde(rename = "lessonNumbers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson_numbers: Option<Vec<i64>>,
    #[serde(rename = "workType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_type: Option<WorkTypeEnum>,
}
impl std::fmt::Display for GetWorksGroupDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
