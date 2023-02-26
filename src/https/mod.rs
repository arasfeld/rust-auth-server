use anyhow::Context;
use axum::{routing::{get, post}, Extension, Router};
use axum_server::tls_rustls::RustlsConfig;
use oauth2::basic::BasicClient;
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use crate::config::Config;

mod error;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `State<AppState>` to a handler function's
/// parameters.
#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    db: PgPool,
    google_client: BasicClient,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port_https));

    // configure certificate and private key used by https
    let tls_config = RustlsConfig::from_pem_file(
    "certs/cert.pem",
    "certs/key.pem",
    ).await?;

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(middleware::cors()))
        .into_inner();

    let google_client =
        middleware::oauth_google::get_client(&config.google_client_id, &config.google_client_secret);

    let app_state = AppState {
        config: Arc::new(config),
        db,
        google_client,
    };

    // build our application with a route
    let app = Router::new()
        .route("/auth/google", get(routes::google::login))
        .route("/auth/google/callback", get(routes::google::callback))
        .route("/auth/login", post(routes::login::login))
        .route("/auth/register", post(routes::register::register))
        .layer(middleware_stack)
        .with_state(app_state);

    // run it
    tracing::debug!("listening on {}", addr);
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await
        .context("error running HTTPS server")
}
