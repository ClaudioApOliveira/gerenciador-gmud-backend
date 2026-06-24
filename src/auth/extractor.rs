use std::{future::Ready, future::ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload, web::Data};

use crate::user::models::UserRole;
use crate::{AppState, errors::api_error::ApiError};

use super::services::{decode_jwt, parse_user_role};

pub const AUTH_COOKIE_NAME: &str = "gmud_access_token";
pub const REFRESH_COOKIE_NAME: &str = "gmud_refresh_token";

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub username: String,
    pub role: UserRole,
}

pub fn require_roles(user: &AuthenticatedUser, allowed: &[UserRole]) -> Result<(), ApiError> {
    if allowed.contains(&user.role) {
        Ok(())
    } else {
        Err(ApiError::Unauthorized(
            "voce nao tem permissao para acessar este recurso".to_string(),
        ))
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let Some(state) = req.app_data::<Data<AppState>>() else {
            return ready(Err(ApiError::Internal(
                "estado da aplicacao nao encontrado".to_string(),
            )));
        };

        let Some(cookie) = req.cookie(AUTH_COOKIE_NAME) else {
            return ready(Err(ApiError::Unauthorized(
                "cookie de autenticacao ausente".to_string(),
            )));
        };

        match decode_jwt(&state.config, cookie.value()) {
            Ok(claims) => match parse_user_role(&claims.role) {
                Ok(role) => ready(Ok(Self {
                    username: claims.sub,
                    role,
                })),
                Err(err) => ready(Err(err)),
            },
            Err(err) => ready(Err(err)),
        }
    }
}
