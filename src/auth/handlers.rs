use actix_web::{
    HttpResponse,
    cookie::{Cookie, SameSite, time::Duration},
    get, post,
    web::{Data, Json},
};

use crate::{AppState, errors::api_error::ApiError, shared::api_response::ok};

use super::{
    dtos::{AuthUserDto, LoginRequestDto, LoginResponseDto},
    extractor::{AUTH_COOKIE_NAME, AuthenticatedUser, REFRESH_COOKIE_NAME},
    services::{
        create_jwt, create_refresh_jwt, decode_refresh_jwt, parse_user_role, verify_user_login,
    },
};

#[post("/login")]
pub async fn login(
    state: Data<AppState>,
    payload: Json<LoginRequestDto>,
) -> Result<HttpResponse, ApiError> {
    let role = verify_user_login(
        &state.config,
        &state.db,
        &payload.username,
        &payload.password,
    )
    .await?;
    let token = create_jwt(&state.config, &payload.username, &role)?;
    let refresh_token = create_refresh_jwt(&state.config, &payload.username, &role)?;

    let cookie = Cookie::build(AUTH_COOKIE_NAME, token)
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    let refresh_cookie = Cookie::build(REFRESH_COOKIE_NAME, refresh_token)
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/api/v1/auth/refresh")
        .max_age(Duration::seconds(
            (state.config.jwt_refresh_exp_hours as i64) * 3600,
        ))
        .finish();

    let body = LoginResponseDto {
        message: "autenticado com sucesso".to_string(),
        user: AuthUserDto {
            username: payload.username.clone(),
            role,
        },
    };

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .cookie(refresh_cookie)
        .json(ok("login realizado", body)))
}

#[post("/refresh")]
pub async fn refresh(
    state: Data<AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let refresh_cookie = req
        .cookie(REFRESH_COOKIE_NAME)
        .ok_or_else(|| ApiError::Unauthorized("refresh token ausente".to_string()))?;

    let claims = decode_refresh_jwt(&state.config, refresh_cookie.value())?;
    let role = parse_user_role(&claims.role)?;

    let new_access_token = create_jwt(&state.config, &claims.sub, &role)?;
    let access_cookie = Cookie::build(AUTH_COOKIE_NAME, new_access_token)
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok(HttpResponse::Ok().cookie(access_cookie).json(ok(
        "token renovado com sucesso",
        serde_json::json!({ "username": claims.sub, "role": role }),
    )))
}

#[post("/logout")]
pub async fn logout(state: Data<AppState>) -> Result<HttpResponse, ApiError> {
    let cookie = Cookie::build(AUTH_COOKIE_NAME, "")
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::seconds(0))
        .finish();

    let refresh_cookie = Cookie::build(REFRESH_COOKIE_NAME, "")
        .http_only(true)
        .secure(state.config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/api/v1/auth/refresh")
        .max_age(Duration::seconds(0))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .cookie(refresh_cookie)
        .json(ok("logout realizado", serde_json::json!({}))))
}

#[get("/me")]
pub async fn me(user: AuthenticatedUser) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(ok(
        "usuario autenticado",
        AuthUserDto {
            username: user.username,
            role: user.role,
        },
    )))
}
