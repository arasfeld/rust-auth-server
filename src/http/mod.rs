use anyhow::Context;
use axum::{routing::{get, post}, Extension, Router};
use oauth2::basic::BasicClient;
use sqlx::PgPool;
use std::{net::{IpAddr, Ipv6Addr, SocketAddr}, str::FromStr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::Config;

mod error;
mod middleware;
mod repositories;
mod routes;
mod services;
mod types;
mod utils;

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `State<AppState>` to a handler function's
/// parameters.
#[derive(Clone)]
pub struct AppState {
    config: Arc<Config>,
    google_client: BasicClient,
    services: Arc<middleware::services::Services>,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust-auth-server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build the address to bind the server to
    let addr = SocketAddr::from((
        IpAddr::from_str(&config.host).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        config.port
    ));

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(middleware::cors()))
        .into_inner();

    let google_client =
        middleware::oauth_google::get_client(&config.google_client_id, &config.google_client_secret);

    let services = middleware::services::build(db);

    let app_state = AppState {
        config: Arc::new(config),
        google_client,
        services: Arc::new(services)
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
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("failed to bind to address")?;
    
    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}
