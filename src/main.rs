use axum::{routing::{get, post}, Extension, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

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
        .layer(Extension(middleware::postgres::get_pool().await))
        .layer(Extension(middleware::oauth_google::get_client()))
        .into_inner();

    // build our application with a route
    let app = Router::new()
        .route("/auth/google", get(handlers::google::login))
        .route("/auth/google/callback", get(handlers::google::callback))
        .route("/auth/login", post(handlers::login::login))
        .route("/auth/register", post(handlers::register::register))
        .layer(middleware_stack);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
