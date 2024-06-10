use std::sync::Arc;

use tonic::codegen::http::{HeaderValue, Request};
use tonic::{async_trait, transport::Body, Status};
use tonic_middleware::RequestInterceptor;
use tracing::{debug, info};

use crate::{
    domain::models::user_role::UserWithRoles, infrastructure::data::context::app_ctx::AppCtx,
};

#[async_trait]
pub trait AuthServiceInterceptor: Send + Sync + 'static {
    async fn get_user_data(&self, token: String) -> Result<UserWithRoles, Status>;
}

#[derive(Clone)]
pub struct AuthServiceInterceptorImpl {
    pub ctx: Arc<AppCtx>,
}

#[async_trait]
impl AuthServiceInterceptor for AuthServiceInterceptorImpl {
    async fn get_user_data(&self, token: String) -> Result<UserWithRoles, Status> {
        debug!("[gRPC] Get user data from token: {}", token);
        let user = self
            .ctx
            .auth_usecase
            .get_user_in_context(token)
            .await
            .map_err(|(_, err)| {
                Status::unauthenticated(format!("Invalid token: {}", err.to_string()))
            })?;

        Ok(user)
    }
}

#[derive(Clone)]
pub struct AuthInterceptor<A: AuthServiceInterceptor> {
    pub ctx: Arc<A>,
}

#[async_trait]
impl<A: AuthServiceInterceptor> RequestInterceptor for AuthInterceptor<A> {
    async fn intercept(&self, mut req: Request<Body>) -> Result<Request<Body>, Status> {
        info!("[gRPC] Start Auth Interceptor");
        let whitelisted_endpoints = vec!["/auth.AuthService/Login", "/auth.AuthService/Register"];
        let req_path = &req.uri().path();

        if whitelisted_endpoints.iter().any(|e| req_path.contains(e)) {
            info!(
                "[gRPC] Request path is whitelisted and will be passed through - {}",
                req_path
            );
            return Ok(req);
        }
        info!(
            "[gRPC] Request path is not whitelisted and will be intercepted - {}",
            req_path
        );

        match req.headers().get("authorization").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                // remove Bearer prefix
                let token = token.strip_prefix("Bearer ").ok_or_else(|| {
                    Status::unauthenticated("Invalid token: token is not prefixed with Bearer")
                })?;

                let user = self.ctx.get_user_data(token.to_string()).await?;

                // Set user id in header, so it can be used in grpc services through tonic::Request::metadata()
                let user_id_header_value = HeaderValue::from_str(&user.user.id.to_string())
                    .map_err(|_e| Status::internal("Failed to convert user_id to header value"))?;

                req.headers_mut().insert("user_id", user_id_header_value);
                req.extensions_mut().insert(user);

                Ok(req)
            }
            _ => Err(Status::unauthenticated("Unauthenticated")),
        }
    }
}
