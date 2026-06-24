use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub request_id: String,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub page: u64,
    pub limit: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiPaginatedResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub request_id: String,
    pub message: String,
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

pub fn request_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();

    format!("req-{nanos}")
}

pub fn ok<T>(message: impl Into<String>, data: T) -> ApiResponse<T>
where
    T: Serialize,
{
    ApiResponse {
        success: true,
        request_id: request_id(),
        message: message.into(),
        data,
    }
}

pub fn paginated<T>(
    message: impl Into<String>,
    data: Vec<T>,
    pagination: PaginationMeta,
) -> ApiPaginatedResponse<T>
where
    T: Serialize,
{
    ApiPaginatedResponse {
        success: true,
        request_id: request_id(),
        message: message.into(),
        data,
        pagination,
    }
}
