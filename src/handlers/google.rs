use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    Extension
};
use oauth2::{
    basic::BasicClient,
    reqwest::async_http_client,
    AuthorizationCode, CsrfToken, Scope, TokenResponse
};

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Profile {
    email: String,
    family_name: String,
    given_name: String,
    id: String,
    locale: String,
    name: String,
    picture: String,
}

pub async fn login(Extension(google_oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    let (authorize_url, _csrf_state) = google_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(authorize_url.as_ref())
}

pub async fn callback(
    Query(query): Query<AuthRequest>,
    Extension(google_oauth_client): Extension<BasicClient>
) -> impl IntoResponse {
    let token = google_oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user profile
    let client = reqwest::Client::new();
    let _profile: Profile = client
        .get("https://www.googleapis.com/auth/userinfo.profile")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<Profile>()
        .await
        .unwrap();
    
    Redirect::to("/")
}
