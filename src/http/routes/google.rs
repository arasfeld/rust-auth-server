use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};

use crate::http::{
    services::oauth_registration_service::link_or_register_oauth_user,
    models::oauth_profile::{GoogleOAuthProfile, OAuthProfile},
    AppState
};

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
    let (authorize_url, _csrf_state) = state.google_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    Redirect::to(authorize_url.as_ref())
}

pub async fn callback(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let token = state.google_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user profile
    let client = reqwest::Client::new();
    let google_profile = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<GoogleOAuthProfile>()
        .await
        .unwrap();

    let profile: OAuthProfile = google_profile.into();

    let user = link_or_register_oauth_user(&state.db, None, "google", &profile.id, &profile)
        .await
        .unwrap();

    Json(user)
}
