use serde::{Serialize, Deserialize};
use super::{GetWorksGroupDto, Lesson};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWorksForAllLessonsGroupDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lesson: Option<Lesson>,
    #[serde(rename = "worksList")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<GetWorksGroupDto>>,
}
impl std::fmt::Display for GetWorksForAllLessonsGroupDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
