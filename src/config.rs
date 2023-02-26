/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.#[derive(clap::Parser)]
/// 
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
#[derive(clap::Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[arg(long, env)]
    pub database_url: String,

    /// Google OAuth client ID
    #[arg(long, env)]
    pub google_client_id: String,

    /// Google OAuth client secret
    #[arg(long, env)]
    pub google_client_secret: String,

    /// The secret used for encoding JWT tokens
    #[arg(long, env)]
    pub jwt_secret: String,

    /// The port to serve the HTTP redirect server
    #[arg(long, env)]
    pub port_http: u16,

    /// The port to serve the HTTPS application server
    #[arg(long, env)]
    pub port_https: u16,
}
