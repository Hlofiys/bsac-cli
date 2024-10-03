use serde::{Serialize, Deserialize};
use super::EducationTypes;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub course: Option<i64>,
    #[serde(rename = "educationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_type: Option<EducationTypes>,
    #[serde(rename = "groupNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removable: Option<bool>,
    #[serde(rename = "semesterEnd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_end: Option<chrono::NaiveDate>,
    #[serde(rename = "semesterStart")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_start: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
