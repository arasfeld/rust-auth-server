use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::https::AppState;
use crate::https::models::user::User;
use crate::https::services::login_service;
use crate::https::utils::jwt;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

pub async fn login(
    Query(query): Query<LoginRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user = login_service::login(&state.db, &query.username, &query.password).await.unwrap();

    let jwt_secret = state.config.jwt_secret.to_owned();
    let token = jwt::sign(user.id, jwt_secret).unwrap();

    Json(LoginResponse { token, user })
}
