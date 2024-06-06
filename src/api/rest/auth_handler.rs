use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde_json::{json, Value};

use crate::{application::dtos::{auth_dto::{LoginRequest, LoginResponse}, user_dto::CreateUserReq}, infrastructure::{data::context::app_ctx::AppCtx, errors::usecase_error::UsecaseError}};

pub fn auth_routes() -> Router<Arc<AppCtx>> {
    Router::new()
        .route("/api/v1/auth", post(authenticate))
        .route("/api/v1/auth/register", post(auth_user_register))
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