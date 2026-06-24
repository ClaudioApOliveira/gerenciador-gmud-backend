use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use thiserror::Error;

use crate::shared::api_response::request_id;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("requisicao invalida: {0}")]
    BadRequest(String),
    #[error("recurso nao encontrado: {0}")]
    NotFound(String),
    #[error("erro de validacao: {0}")]
    Validation(String),
    #[error("nao autorizado: {0}")]
    Unauthorized(String),
    #[error("erro de banco de dados")]
    Database(#[from] mongodb::error::Error),
    #[error("erro interno: {0}")]
    Internal(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Database(_) | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_type = match self {
            Self::BadRequest(_) => "bad_request",
            Self::NotFound(_) => "not_found",
            Self::Validation(_) => "validation_error",
            Self::Unauthorized(_) => "unauthorized",
            Self::Database(_) => "database_error",
            Self::Internal(_) => "internal_error",
        };

        HttpResponse::build(self.status_code()).json(json!({
            "success": false,
            "requestId": request_id(),
            "message": self.to_string(),
            "errorType": error_type,
        }))
    }
}
