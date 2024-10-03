use serde::{Serialize, Deserialize};
use super::{GetScheduleForOneGroup, Group};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScheduleForListOfGroups {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Vec<GetScheduleForOneGroup>>,
}
impl std::fmt::Display for GetScheduleForListOfGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
