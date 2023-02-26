use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_auth_server::config::Config;
use rust_auth_server::{http, https};

#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rust-auth-server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Parse our configuration from the environment.
    // This will exit with a help message if something is wrong.
    let config = Config::parse();

    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await?;

    // spawn a second server to redirect http requests to this server
    tokio::spawn(http::redirect_http_to_https(config.port_http, config.port_https));

    // Finally, we spin up our API.
    https::serve(config, db).await?;

    Ok(())
}
