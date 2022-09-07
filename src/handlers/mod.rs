use axum::{routing::get, Extension, Router};

mod google;

pub fn auth_router() -> Router {
    let google_client = google::get_client();

    Router::new()
        .route("/google", get(google::google_auth))
        .route("/google/callback", get(google::google_auth_callback))
        .layer(Extension(google_client))
}
