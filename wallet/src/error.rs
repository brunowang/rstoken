use axum::response::{IntoResponse, Response};
use axum::{http::StatusCode, Json};
use serde_json::json;
pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_response = json!({
            "status": "error",
            "message": format!("Server Error: {}", self.0),
        });
        (StatusCode::OK, Json(error_response)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
