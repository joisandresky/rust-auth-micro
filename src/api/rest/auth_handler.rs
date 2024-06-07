use std::sync::Arc;

use axum::{extract::State, http::StatusCode, middleware, routing::{get, post}, Extension, Json, Router};
use serde_json::{json, Value};

use crate::{api::middleware::auth::auth_mw, application::dtos::{auth_dto::{LoginRequest, LoginResponse}, user_dto::CreateUserReq}, domain::models::user_role::UserWithRoles, infrastructure::{data::context::app_ctx::AppCtx, errors::usecase_error::UsecaseError}};

pub fn auth_routes(ctx: Arc<AppCtx>) -> Router<Arc<AppCtx>> {
    Router::new()
        .route("/api/v1/auth", post(authenticate))
        .route("/api/v1/auth/register", post(auth_user_register))
        .route("/api/v1/auth/me", get(me).route_layer(middleware::from_fn_with_state(ctx.clone(), auth_mw)))
        .route("/api/v1/auth/logout", get(logout).route_layer(middleware::from_fn_with_state(ctx.clone(), auth_mw)))
}

pub async fn authenticate(
    State(ctx): State<Arc<AppCtx>>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), UsecaseError> {
    let resp = ctx.auth_usecase.authenticate(req).await?;

    Ok((
        StatusCode::OK,
        Json(resp)
    ))
}

pub async fn auth_user_register(
    State(ctx): State<Arc<AppCtx>>,
    Json(req): Json<CreateUserReq>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    let user = ctx.auth_usecase.register(&ctx.db_pool, req).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": user.id,
        }))
    ))
}

pub async fn me(
    Extension(user): Extension<UserWithRoles>,
) -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": user,
        }))
    )
}

pub async fn logout(
    State(ctx): State<Arc<AppCtx>>,
    Extension(user): Extension<UserWithRoles>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    let _ = ctx.auth_usecase.logout(&user.user.id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true,
        }))
    ))
}