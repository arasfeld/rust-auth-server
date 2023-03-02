use axum::{response::IntoResponse, Json};

use crate::http::models::user::User;

#[derive(Debug, serde::Serialize)]
pub struct MeResponse {
    user: User,
}

pub async fn me(user: User) -> impl IntoResponse {
    Json(MeResponse { user })
}
