use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use std::time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::http::AppState;
use crate::http::error::Error;
use crate::http::types::User;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

pub fn sign(id: Uuid, secret: String) -> Result<String, Error> {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::from_secs(60 * 60 * 24);

    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            sub: id,
            iat: iat.unix_timestamp(),
            exp: exp.unix_timestamp(),
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap())
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .unwrap();
        // Extract postgres pool extension from request
        let state = AppState::from_ref(state);
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();
        // Get the user from the database
        let user = state.services.user_service.get_by_id(&token_data.claims.sub).await?.unwrap();

        Ok(user)
    }
}
