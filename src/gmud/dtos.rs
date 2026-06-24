use serde::{Deserialize, Serialize};
use validator::Validate;

use super::models::{GmudModel, GmudStatus};

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateGmudDto {
    #[validate(length(min = 5, max = 120))]
    pub title: String,
    #[validate(length(min = 3, max = 120))]
    pub project_id: String,
    #[validate(length(min = 1, max = 40))]
    pub spring: String,
    #[validate(length(min = 3, max = 80))]
    pub gmud_type: String,
    #[validate(length(min = 3, max = 80))]
    pub gmud_number: String,
    pub status: GmudStatus,
    #[validate(length(min = 3, max = 120))]
    pub developer: String,
    #[validate(length(min = 3, max = 120))]
    pub approver: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGmudDto {
    #[validate(length(min = 5, max = 120))]
    pub title: Option<String>,
    #[validate(length(min = 3, max = 120))]
    pub project_id: Option<String>,
    #[validate(length(min = 1, max = 40))]
    pub spring: Option<String>,
    #[validate(length(min = 3, max = 80))]
    pub gmud_type: Option<String>,
    #[validate(length(min = 3, max = 80))]
    pub gmud_number: Option<String>,
    #[validate(length(min = 3, max = 120))]
    pub developer: Option<String>,
    #[validate(length(min = 3, max = 120))]
    pub approver: Option<String>,
    pub status: Option<GmudStatus>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GmudResponseDto {
    pub id: String,
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

impl From<GmudModel> for GmudResponseDto {
    fn from(value: GmudModel) -> Self {
        Self {
            id: value.id.map(|id| id.to_hex()).unwrap_or_default(),
            title: value.title,
            project_id: value.project_id,
            spring: value.spring,
            gmud_type: value.gmud_type,
            gmud_number: value.gmud_number,
            developer: value.developer,
            approver: value.approver,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGmudsQueryDto {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<GmudStatus>,
    pub project_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GmudListResponseDto {
    pub items: Vec<GmudResponseDto>,
    pub page: u64,
    pub limit: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

