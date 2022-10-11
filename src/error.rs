use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    InvalidUsernamePassword,
    TooManyAttempts,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::InvalidUsernamePassword => (
                StatusCode::UNAUTHORIZED,
                "the username and/or password used for authentication are invalid"
            ),
            Self::TooManyAttempts => (
                StatusCode::UNAUTHORIZED,
                "the account is locked due to too many failed login attempts"
            ),
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}
