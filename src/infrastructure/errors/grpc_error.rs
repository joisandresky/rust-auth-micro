use tonic::Code;

use super::usecase_error::UsecaseError;

impl From<UsecaseError> for tonic::Status {
    fn from(err: UsecaseError) -> Self {
        let code = match err.code {
            400 => Code::InvalidArgument,
            401 => Code::Unauthenticated,
            404 => Code::NotFound,
            500 => Code::Internal,
            _ => Code::Unknown,
        };

        let message = format!("{}: {}", err.code, err.message);
        tonic::Status::new(code, message)
    }
}