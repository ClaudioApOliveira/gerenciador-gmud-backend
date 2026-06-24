use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GmudStatus {
    Draft,
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmudModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub project_id: String,
    pub spring: String,
    pub gmud_type: String,
    pub gmud_number: String,
    pub developer: String,
    pub approver: String,
    pub status: GmudStatus,
    pub created_at: String,
    pub updated_at: String,
}
