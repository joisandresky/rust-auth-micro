use std::sync::Arc;

use axum::{body::Body, extract::{Request, State}, http::{header, StatusCode}, middleware::Next, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde_json::{json, Value};
use tracing::{error, info, warn};

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
            req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        }).ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "Unauthorized"
                }))
            )
        })?;

    let claim = ctx
        .paseto_maker
        .verify_token(token)
        .map_err(|err| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": err.to_string()
                }))
            )
        })?;

    let mut user_id = claim["sub"].to_string();
    let user_role = claim["aud"].to_string();
    info!("user id {} with Role {}", &user_id, &user_role);

    // remove double quotes from user id
    if user_id.starts_with("\"") {
        user_id = user_id[1..user_id.len()-1].to_owned();
    }

    if ctx.redis_repo.lock().await.get_auth_token(&user_id).await.is_none() {
        error!("token user with id {} is not exist in redis", &user_id);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "Unauthorized"
            }))
        ));
    }

    let existing_user = ctx.redis_repo.lock().await.get_user_data(&user_id).await;

    if let Some(u) = existing_user {
        info!("user with id {} is still exist in redis, use existing one", &u.user.id);

        req.extensions_mut().insert(u);
        return Ok(next.run(req).await)
    }
    warn!("user with id {} is not exist in redis, get user from db", &user_id);

    let user = ctx
        .user_usecase
        .get_by_id_with_roles(&user_id)
        .await
        .map_err(|err| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": err.to_string()
                }))
            )
        })?;

    let _ = ctx.redis_repo.lock().await.set_user_data(user_id, &user).await;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}