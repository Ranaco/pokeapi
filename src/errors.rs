use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    RequestFailed(String),
    ParseError(String),
    UpstreamError,
    InvalidParameter(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::RequestFailed(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::ParseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse response"),
            ApiError::UpstreamError => (StatusCode::BAD_GATEWAY, "Upstream service error"),
            ApiError::InvalidParameter(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
