use std::sync::Arc;

use axum::{
    extract::State,
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use crate::{
    api::middleware::auth::auth_mw,
    application::dtos::{auth_dto::LoginRequest, user_dto::CreateUserReq},
    domain::models::user_role::UserWithRoles,
    infrastructure::{
        data::context::app_ctx::AppCtx, errors::usecase_error::UsecaseError,
        utils::response::SuccessResponse,
    },
};

pub fn auth_routes(ctx: Arc<AppCtx>) -> Router<Arc<AppCtx>> {
    Router::new()
        .route("/", post(authenticate))
        .route("/register", post(auth_user_register))
        .route(
            "/me",
            get(me).route_layer(middleware::from_fn_with_state(ctx.clone(), auth_mw)),
        )
        .route(
            "/logout",
            get(logout).route_layer(middleware::from_fn_with_state(ctx.clone(), auth_mw)),
        )
}

pub async fn authenticate(
    State(ctx): State<Arc<AppCtx>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, UsecaseError> {
    let resp = ctx.auth_usecase.authenticate(req).await?;

    Ok(SuccessResponse::new(200, None, Some(resp)).into_response())
}

pub async fn auth_user_register(
    State(ctx): State<Arc<AppCtx>>,
    Json(req): Json<CreateUserReq>,
) -> Result<impl IntoResponse, UsecaseError> {
    let user = ctx.auth_usecase.register(&ctx.db_pool, req).await?;

    Ok(SuccessResponse::new(
        201,
        Some(String::from("Successfully registered!")),
        Some(user.id),
    )
    .into_response())
}

pub async fn me(Extension(user): Extension<UserWithRoles>) -> impl IntoResponse {
    SuccessResponse::new(200, None, Some(user)).into_response()
}

pub async fn logout(
    State(ctx): State<Arc<AppCtx>>,
    Extension(user): Extension<UserWithRoles>,
) -> Result<impl IntoResponse, UsecaseError> {
    let _ = ctx.auth_usecase.logout(&user.user.id).await?;

    Ok(SuccessResponse::<u16>::new(200, None, None).into_response())
}
