use serde::{Deserialize, Serialize};

use crate::user::models::UserRole;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthUserDto {
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub message: String,
    pub user: AuthUserDto,
}

