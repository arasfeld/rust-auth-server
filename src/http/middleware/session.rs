use axum::{async_trait, extract::{FromRef, FromRequestParts}, http::request::Parts};

use crate::http::{
    error::Error,
    models::user::User,
    repositories::user_repository,
    services::session_service,
    AppState
};

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let maybe_user_id = session_service::get(app_state.session_store, parts).await;

        if let Some(user_id) = maybe_user_id {
            return user_repository::get_by_id(&app_state.db, user_id)
                .await
                .map_err(|_| Error::NotFound);
        }

        Err(Error::Unauthorized)
    }
}
