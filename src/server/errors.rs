use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type ApiResult<T> = Result<T, ApiError>;

pub struct ApiError {
    code: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

impl From<tapo::Error> for ApiError {
    fn from(value: tapo::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, format!("{value}"))
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, format!("{value}"))
    }
}
