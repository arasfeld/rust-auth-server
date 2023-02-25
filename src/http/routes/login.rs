use axum::{
    extract::Query,
    response::IntoResponse,
    Extension, Json,
};
use sqlx::PgPool;

use crate::config::Config;
use crate::http::models::user::User;
use crate::http::services::login_service;
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
    Extension(config): Extension<Config>,
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    let user = login_service::login(&db, &query.username, &query.password).await.unwrap();

    let token = jwt::sign(user.id, config.jwt_secret.to_owned()).unwrap();

    Json(LoginResponse { token, user })
}
