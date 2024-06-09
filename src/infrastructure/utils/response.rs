use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub code: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> SuccessResponse<T>
where
    T: Serialize,
{
    pub fn new(code: u16, message: Option<String>, data: Option<T>) -> Self {
        SuccessResponse {
            success: true,
            code,
            message,
            data,
        }
    }
}

impl<T> IntoResponse for SuccessResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let json_body = axum::Json(self);

        // Convert Json to Response
        let mut response = json_body.into_response();

        // Set the correct status code
        *response.status_mut() = code;

        response
    }
}
