use serde::{Serialize, Deserialize};
use super::GetWorksGroupDto;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetKtpWorksDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated: Option<bool>,
    #[serde(rename = "worksList")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<GetWorksGroupDto>>,
}
impl std::fmt::Display for GetKtpWorksDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
