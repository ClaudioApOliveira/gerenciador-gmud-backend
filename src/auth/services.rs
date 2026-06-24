use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::services::{find_user_by_name, verify_user_password};
use crate::{config::AppConfig, errors::api_error::ApiError, user::models::UserRole};

pub const ACCESS_TOKEN_CLAIM_TYPE: &str = "access";
pub const REFRESH_TOKEN_CLAIM_TYPE: &str = "refresh";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub role: String,
    pub token_type: String,
    pub jti: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn verify_credentials(
    config: &AppConfig,
    username: &str,
    password: &str,
) -> Result<(), ApiError> {
    if username == config.auth_username && password == config.auth_password {
        Ok(())
    } else {
        Err(ApiError::Unauthorized("credenciais invalidas".to_string()))
    }
}

pub async fn verify_user_login(
    config: &AppConfig,
    db: &mongodb::Database,
    username: &str,
    password: &str,
) -> Result<UserRole, ApiError> {
    if let Some(user) = find_user_by_name(db, username).await? {
        let password_hash = user
            .password_hash
            .ok_or_else(|| ApiError::Unauthorized("usuario sem senha configurada".to_string()))?;

        if verify_user_password(password, &password_hash)? {
            return Ok(user.role);
        }

        return Err(ApiError::Unauthorized("credenciais invalidas".to_string()));
    }

    verify_credentials(config, username, password)?;
    parse_user_role(&config.auth_role)
}

pub fn parse_user_role(value: &str) -> Result<UserRole, ApiError> {
    match value.trim().to_lowercase().as_str() {
        "developer" => Ok(UserRole::Developer),
        "approver" => Ok(UserRole::Approver),
        "admin" => Ok(UserRole::Admin),
        _ => Err(ApiError::Validation("role invalido".to_string())),
    }
}

pub fn create_jwt(config: &AppConfig, username: &str, role: &UserRole) -> Result<String, ApiError> {
    create_token(
        config,
        username,
        role,
        ACCESS_TOKEN_CLAIM_TYPE,
        config.jwt_access_exp_minutes,
        true,
    )
}

pub fn create_refresh_jwt(
    config: &AppConfig,
    username: &str,
    role: &UserRole,
) -> Result<String, ApiError> {
    create_token(
        config,
        username,
        role,
        REFRESH_TOKEN_CLAIM_TYPE,
        config.jwt_refresh_exp_hours,
        false,
    )
}

fn create_token(
    config: &AppConfig,
    username: &str,
    role: &UserRole,
    token_type: &str,
    exp_value: u64,
    is_minutes: bool,
) -> Result<String, ApiError> {
    let now = Utc::now().timestamp() as usize;
    let exp = if is_minutes {
        now + (exp_value as usize * 60)
    } else {
        now + (exp_value as usize * 3600)
    };
    let role_str = match role {
        UserRole::Developer => "developer",
        UserRole::Approver => "approver",
        UserRole::Admin => "admin",
    };

    let claims = JwtClaims {
        sub: username.to_string(),
        role: role_str.to_string(),
        token_type: token_type.to_string(),
        jti: Uuid::new_v4().to_string(),
        iat: now,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::Internal("falha ao gerar token".to_string()))
}

pub fn decode_jwt(config: &AppConfig, token: &str) -> Result<JwtClaims, ApiError> {
    let claims = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::Unauthorized("token invalido ou expirado".to_string()))?;

    if claims.token_type != ACCESS_TOKEN_CLAIM_TYPE {
        return Err(ApiError::Unauthorized(
            "token de acesso invalido".to_string(),
        ));
    }

    Ok(claims)
}

pub fn decode_refresh_jwt(config: &AppConfig, token: &str) -> Result<JwtClaims, ApiError> {
    let claims = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::Unauthorized("refresh token invalido ou expirado".to_string()))?;

    if claims.token_type != REFRESH_TOKEN_CLAIM_TYPE {
        return Err(ApiError::Unauthorized("refresh token invalido".to_string()));
    }

    Ok(claims)
}
