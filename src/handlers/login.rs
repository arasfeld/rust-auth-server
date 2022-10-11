use axum::{
    extract::Query,
    response::IntoResponse,
    Extension, Json,
};
use sqlx::PgPool;

use crate::models::user::User;
use crate::services::login_service;
use crate::utils::jwt;

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
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    let user = login_service::login(&db, &query.username, &query.password).await.unwrap();

    let token = jwt::sign(user.id).unwrap();

    Json(LoginResponse { token, user })
}
