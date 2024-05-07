use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;


pub enum CustomError {
    BadRequest,
    InvalidAccessCrendentials,
    InternalServerError,
    UserNotFound,
    InvalidCrendentials
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            CustomError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            CustomError::BadRequest=> (StatusCode::BAD_REQUEST, "Bad Request"),
            CustomError::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
            CustomError::InvalidCrendentials => (StatusCode::NOT_ACCEPTABLE, "Invalid Crendentials"),
            CustomError::InvalidAccessCrendentials => (StatusCode::NOT_ACCEPTABLE, "Insufficient  Access Crendentials"),
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}