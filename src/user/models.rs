use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Developer,
    Approver,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub role: UserRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
}

