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
    let user = state.services.login_service.login(&query.username, &query.password).await.unwrap();

    let jwt_secret = state.config.jwt_secret.to_owned();
    let token = jwt::sign(user.id, jwt_secret).unwrap();

    Json(LoginResponse { token, user })
}
