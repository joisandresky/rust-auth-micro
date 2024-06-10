use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde_json::{json, Value};
use tracing::info;

use crate::infrastructure::data::context::app_ctx::AppCtx;

pub async fn auth_mw(
    cookie_jar: CookieJar,
    State(ctx): State<Arc<AppCtx>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    info!("Start Auth Middleware");

    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "Unauthorized"
                })),
            )
        })?;

    let user = ctx.auth_usecase.get_user_in_context(token).await?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
