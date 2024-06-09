use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{
    application::dtos::role_dto::CreateOrUpdateRoleReq,
    infrastructure::{
        data::context::app_ctx::AppCtx, errors::usecase_error::UsecaseError,
        utils::response::SuccessResponse,
    },
};

pub fn role_routes() -> Router<Arc<AppCtx>> {
    Router::new()
        .route("/", get(get_all_roles).post(create_role))
        .route(
            "/:id",
            get(get_role_by_id).put(update_role).delete(delete_role),
        )
}

pub async fn get_all_roles(
    State(ctx): State<Arc<AppCtx>>,
) -> Result<impl IntoResponse, UsecaseError> {
    let roles = ctx.role_usecase.get_all().await?;

    Ok(SuccessResponse::new(200, None, Some(roles)).into_response())
}

pub async fn get_role_by_id(
    State(ctx): State<Arc<AppCtx>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, UsecaseError> {
    let role = ctx.role_usecase.get_by_id(id).await?;

    Ok(SuccessResponse::new(200, None, Some(role)).into_response())
}

pub async fn create_role(
    State(ctx): State<Arc<AppCtx>>,
    Json(req): Json<CreateOrUpdateRoleReq>,
) -> Result<impl IntoResponse, UsecaseError> {
    let role = ctx.role_usecase.create(req).await?;

    Ok(SuccessResponse::new(
        201,
        Some("Successfully created a new role".to_string()),
        Some(role.id),
    )
    .into_response())
}

pub async fn update_role(
    State(ctx): State<Arc<AppCtx>>,
    Path(id): Path<String>,
    Json(req): Json<CreateOrUpdateRoleReq>,
) -> Result<impl IntoResponse, UsecaseError> {
    let role = ctx.role_usecase.update_by_id(id, req).await?;

    Ok(SuccessResponse::new(
        200,
        Some(format!("Successfully updated role {}", role.id)),
        Some(role.id),
    )
    .into_response())
}

pub async fn delete_role(
    State(ctx): State<Arc<AppCtx>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, UsecaseError> {
    ctx.role_usecase.delete_by_id(id).await?;

    Ok(SuccessResponse::<u16>::new(200, None, None).into_response())
}
