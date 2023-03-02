use axum::{
    extract::{Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::IntoResponse,
    Json
};

use crate::http::AppState;
use crate::http::models::user::User;
use crate::http::services::{login_service, session_service};

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    user: User,
}

pub async fn login(
    Query(query): Query<LoginRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user = login_service::login(&state.db, &query.username, &query.password).await.unwrap();

    let cookie = session_service::create(state.session_store, &user.id).await;

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Json(LoginResponse { user }))
}
