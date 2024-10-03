use serde::{Serialize, Deserialize};
use super::{GetKtpDto, HttpStatusCode};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetKtpDtoListServiceResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GetKtpDto>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_code: Option<HttpStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
impl std::fmt::Display for GetKtpDtoListServiceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
