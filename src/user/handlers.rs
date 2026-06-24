use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    auth::extractor::{require_roles, AuthenticatedUser},
    errors::api_error::ApiError,
    shared::api_response::ok,
    user::models::UserRole,
    AppState,
};

use super::{dtos::CreateUserDto, services};

#[post("")]
pub async fn create_user(
    state: Data<AppState>,
    user: AuthenticatedUser,
    payload: Json<CreateUserDto>,
) -> Result<HttpResponse, ApiError> {
    require_roles(&user, &[UserRole::Admin])?;
    let created = services::create_user(&state.db, payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(ok("usuario criado com sucesso", created)))
}

#[get("")]
pub async fn list_users(
    state: Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, ApiError> {
    require_roles(&user, &[UserRole::Admin, UserRole::Approver])?;
    let users = services::list_users(&state.db).await?;
    Ok(HttpResponse::Ok().json(ok("usuarios listados com sucesso", users)))
}

