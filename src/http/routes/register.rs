use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::http::AppState;
use crate::http::models::user::User;
use crate::http::services::registration_service;
use crate::http::utils::jwt;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct RegisterResponse {
    token: String,
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

    let jwt_secret = state.config.jwt_secret.to_owned();
    let token = jwt::sign(user.id, jwt_secret).unwrap();

    Json(RegisterResponse { token, user })
}
