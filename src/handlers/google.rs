use axum::{
  extract::Query,
  response::{IntoResponse, Redirect},
  Extension
};
use oauth2::{
  basic::BasicClient,
  reqwest::async_http_client,
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
  RedirectUrl, RevocationUrl, Scope, TokenUrl
};

pub fn get_client() -> BasicClient {
    let client_id = ClientId::new(
        std::env::var("GOOGLE_CLIENT_ID")
            .expect("Missing the GOOGLE_CLIENT_ID environment variable.")
    );
    let client_secret = ClientSecret::new(
        std::env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable.")
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_uri = RedirectUrl::new("http://localhost:3000/auth/google/callback".to_string())
        .expect("Invalid redirect URL");
    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
        .expect("Invalid revocation endpoint URL");

    BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_uri)
    .set_revocation_uri(revocation_url)
}

pub async fn google_auth(Extension(google_oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    let (authorize_url, _csrf_state) = google_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    Redirect::to(authorize_url.as_ref())
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
  code: String,
  state: String,
}

pub async fn google_auth_callback(
  Query(query): Query<AuthRequest>,
  Extension(google_oauth_client): Extension<BasicClient>
) -> impl IntoResponse {
    let _token = google_oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();
    
    Redirect::to("/")
}
