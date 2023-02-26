use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError,
};
use std::net::SocketAddr;

pub async fn redirect_http_to_https(http_port: u16, https_port: u16) {
  fn make_https(host: String, uri: Uri, http_port: u16, https_port: u16) -> Result<Uri, BoxError> {
      let mut parts = uri.into_parts();

      parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

      if parts.path_and_query.is_none() {
          parts.path_and_query = Some("/".parse().unwrap());
      }

      let https_host = host.replace(&http_port.to_string(), &https_port.to_string());
      parts.authority = Some(https_host.parse()?);

      Ok(Uri::from_parts(parts)?)
  }

  let redirect = move |Host(host): Host, uri: Uri| async move {
      match make_https(host, uri, http_port, https_port) {
          Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
          Err(error) => {
              tracing::warn!(%error, "failed to convert URI to HTTPS");
              Err(StatusCode::BAD_REQUEST)
          }
      }
  };

  let addr = SocketAddr::from(([127, 0, 0, 1], http_port));
  tracing::debug!("HTTP redirect listening on {}", addr);

  axum::Server::bind(&addr)
      .serve(redirect.into_make_service())
      .await
      .unwrap();
}
