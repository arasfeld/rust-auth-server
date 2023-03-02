use async_session::{MemoryStore, Session, SessionStore};
use axum::{extract::TypedHeader, headers::Cookie, http::request::Parts, RequestPartsExt};
use uuid::Uuid;

const SESSION_COOKIE_NAME: &str = "session";

pub async fn create(store: MemoryStore, id: &Uuid) -> String {
    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("id", id).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.store_session(session).await.unwrap().unwrap();

    // Build the cookie
    format!("{}={}; SameSite=Lax; Path=/", SESSION_COOKIE_NAME, cookie)
}

pub async fn get(store: MemoryStore, parts: &mut Parts) -> Option<Uuid> {
    let cookie = parts
        .extract::<Option<TypedHeader<Cookie>>>()
        .await
        .unwrap();

    let session_cookie = cookie
        .as_ref()
        .and_then(|cookie| cookie.get(SESSION_COOKIE_NAME));

    store.load_session(session_cookie.unwrap().to_owned())
        .await
        .unwrap()
        .and_then(|session| session.get::<Uuid>("id"))
}
