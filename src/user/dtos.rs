use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::{UserModel, UserRole};

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 120))]
    pub name: String,
    pub role: UserRole,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseDto {
    pub id: String,
    pub name: String,
    pub role: UserRole,
}

impl From<UserModel> for UserResponseDto {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id.map(|id| id.to_hex()).unwrap_or_default(),
            name: value.name,
            role: value.role,
        }
    }
}

