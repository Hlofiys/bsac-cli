use serde::{Serialize, Deserialize};
use super::{GetKtpWorksDto, Ktp};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKtpDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ktp: Option<Ktp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub works: Option<GetKtpWorksDto>,
}
impl std::fmt::Display for GetKtpDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
