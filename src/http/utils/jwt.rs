use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;
use std::time::Duration;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::config::Config;
use crate::http::models::user::User;
use crate::http::{error::Error, repositories::user_repository};

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
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .unwrap();
        // Extract postgres pool extension from request
        let Extension(pool) = Extension::<PgPool>::from_request(req).await.unwrap();
        let Extension(config) = Extension::<Config>::from_request(req).await.unwrap();
        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();
        // Get the user from the database
        let user = user_repository::get_by_id(&pool, token_data.claims.sub).await?;

        Ok(user)
    }
}
