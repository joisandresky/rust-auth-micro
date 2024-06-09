use std::sync::Arc;

use crate::application::dtos::auth_dto::LoginRequest as appLoginRequest;
use auth_proto::{
    auth_service_server::AuthService, LoginRequest, LoginResponse, LogoutRequest, LogoutResponse,
    MeRequest, MeResponse, RegisterRequest, RegisterResponse,
};
use tonic::{Request, Response, Status};

use crate::infrastructure::data::context::app_ctx::AppCtx;

pub mod auth_proto {
    tonic::include_proto!("auth");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("auth_descriptor");
}

pub struct GrpcAuthService {
    ctx: Arc<AppCtx>,
}

impl GrpcAuthService {
    pub fn new(ctx: Arc<AppCtx>) -> Self {
        Self { ctx }
    }
}

#[allow(unused)]
#[tonic::async_trait]
impl AuthService for GrpcAuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        let login_req: appLoginRequest = req.into();

        let resp = self.ctx.auth_usecase.authenticate(login_req).await?;

        Ok(Response::new(resp.into()))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        todo!()
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        todo!()
    }

    async fn me(&self, request: Request<MeRequest>) -> Result<Response<MeResponse>, Status> {
        todo!()
    }
}
