use crate::prelude::*;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<()>,
}