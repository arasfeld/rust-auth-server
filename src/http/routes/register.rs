use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::http::AppState;
use crate::http::types::User;
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
    let user = state.services.registration_service.register_user(
        &query.username,
        &query.email,
        &query.password,
    ).await.unwrap();

    let jwt_secret = state.config.jwt_secret.to_owned();
    let token = jwt::sign(user.id, jwt_secret).unwrap();

    Json(RegisterResponse { token, user })
}
