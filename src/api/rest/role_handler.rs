use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{application::dtos::role_dto::CreateOrUpdateRoleReq, infrastructure::{data::context::app_ctx::AppCtx, errors::usecase_error::UsecaseError}};

pub fn role_routes() -> Router<Arc<AppCtx>> {
    Router::new()
       .route("/api/v1/roles", get(get_all_roles).post(create_role))
       .route("/api/v1/roles/:id", get(get_role_by_id).put(update_role).delete(delete_role))
}


pub async fn get_all_roles(
    State(ctx): State<Arc<AppCtx>>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    let roles = ctx.role_usecase.get_all().await?;

    Ok((StatusCode::OK, Json(json!({
        "data": roles
    }))))
}

pub async fn get_role_by_id(
    State(ctx): State<Arc<AppCtx>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    let role = ctx.role_usecase.get_by_id(id).await?;

    Ok((StatusCode::OK, Json(json!({
        "data": role
    }))))
}


pub async fn create_role(
    State(ctx): State<Arc<AppCtx>>,
    Json(req): Json<CreateOrUpdateRoleReq>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    let role = ctx.role_usecase.create(req).await?;

    Ok((StatusCode::OK, Json(json!({
        "success": true,
        "data": role.id,
    }))))
}

pub async fn update_role(
    State(ctx): State<Arc<AppCtx>>,
    Path(id): Path<String>,
    Json(req): Json<CreateOrUpdateRoleReq>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    let role = ctx.role_usecase.update_by_id(id, req).await?;

    Ok((StatusCode::OK, Json(json!({
        "success": true,
        "data": role.id,
    }))))
}

pub async fn delete_role(
    State(ctx): State<Arc<AppCtx>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Value>), UsecaseError> {
    ctx.role_usecase.delete_by_id(id).await?;

    Ok((StatusCode::OK, Json(json!({
        "success": true,
    }))))
}