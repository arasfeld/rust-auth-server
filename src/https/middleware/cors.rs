use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

pub fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
}
