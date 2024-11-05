use crate::prelude::*;
use crate::types::Role;

#[derive(Serialize)]
pub struct RoleRequired {
    pub error: &'static str,
    #[serde(rename = "roleRequired")]
    pub role_required: Role,
}

impl RoleRequired {
    pub fn new(role: Role) -> Self {
        Self {
            error: "You do not have the role required to access this endpoint.",
            role_required: role,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleSelect {
    #[serde(rename = "availableRoles")]
    pub available_roles: Vec<Role>,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub name: String,
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<()>,
}