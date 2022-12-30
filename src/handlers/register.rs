use axum::{
    extract::Query,
    response::IntoResponse,
    Extension, Json,
};
use sqlx::PgPool;

use crate::models::user::User;
use crate::services::registration_service;
use crate::utils::jwt;

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
    Extension(db): Extension<PgPool>,
) -> impl IntoResponse {
    let user = registration_service::register_user(
        &db,
        &query.username,
        &query.email,
        Some(&query.password),
        false
    ).await.unwrap();

    let token = jwt::sign(user.id).unwrap();

    Json(RegisterResponse { token, user })
}
