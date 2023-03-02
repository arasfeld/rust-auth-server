use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::http::AppState;
use crate::http::models::user::User;
use crate::http::services::registration_service;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct RegisterResponse {
    user: User,
}

pub async fn register(
    Query(query): Query<RegisterRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user = registration_service::register_user(
        &state.db,
        &query.username,
        &query.email,
        Some(&query.password),
        false
    ).await.unwrap();

    Json(RegisterResponse { user })
}
