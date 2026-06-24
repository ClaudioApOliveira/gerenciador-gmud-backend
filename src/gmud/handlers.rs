use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse,
};

use crate::{
    auth::extractor::{require_roles, AuthenticatedUser},
    errors::api_error::ApiError,
    shared::api_response::{ok, paginated, PaginationMeta},
    user::models::UserRole,
    AppState,
};

use super::{
    dtos::{CreateGmudDto, ListGmudsQueryDto, UpdateGmudDto},
    services,
};

#[post("")]
pub async fn create_gmud(
    state: Data<AppState>,
    user: AuthenticatedUser,
    payload: Json<CreateGmudDto>,
) -> Result<HttpResponse, ApiError> {
    require_roles(&user, &[UserRole::Admin, UserRole::Developer])?;
    let result = services::create_gmud(&state.db, payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(ok("gmud criada com sucesso", result)))
}

#[get("")]
pub async fn list_gmuds(
    state: Data<AppState>,
    user: AuthenticatedUser,
    query: Query<ListGmudsQueryDto>,
) -> Result<HttpResponse, ApiError> {
    require_roles(
        &user,
        &[UserRole::Admin, UserRole::Developer, UserRole::Approver],
    )?;
    let result = services::list_gmuds(&state.db, query.into_inner()).await?;
    let response = paginated(
        "gmuds listadas com sucesso",
        result.items,
        PaginationMeta {
            page: result.page,
            limit: result.limit,
            total_items: result.total_items,
            total_pages: result.total_pages,
        },
    );
    Ok(HttpResponse::Ok().json(response))
}

#[get("/{id}")]
pub async fn get_gmud_by_id(
    state: Data<AppState>,
    user: AuthenticatedUser,
    id: Path<String>,
) -> Result<HttpResponse, ApiError> {
    require_roles(
        &user,
        &[UserRole::Admin, UserRole::Developer, UserRole::Approver],
    )?;
    let result = services::get_gmud_by_id(&state.db, &id).await?;
    Ok(HttpResponse::Ok().json(ok("gmud encontrada", result)))
}

#[put("/{id}")]
pub async fn update_gmud(
    state: Data<AppState>,
    user: AuthenticatedUser,
    id: Path<String>,
    payload: Json<UpdateGmudDto>,
) -> Result<HttpResponse, ApiError> {
    require_roles(&user, &[UserRole::Admin, UserRole::Developer])?;
    let result = services::update_gmud(&state.db, &id, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ok("gmud atualizada com sucesso", result)))
}

#[delete("/{id}")]
pub async fn delete_gmud(
    state: Data<AppState>,
    user: AuthenticatedUser,
    id: Path<String>,
) -> Result<HttpResponse, ApiError> {
    require_roles(&user, &[UserRole::Admin])?;
    services::delete_gmud(&state.db, &id).await?;
    Ok(HttpResponse::Ok().json(ok(
        "gmud removida com sucesso",
        serde_json::json!({}),
    )))
}

