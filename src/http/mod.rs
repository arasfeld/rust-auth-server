use anyhow::Context;
use axum::{routing::{get, post}, Extension, Router};
use oauth2::basic::BasicClient;
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
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
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust-auth-server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}
