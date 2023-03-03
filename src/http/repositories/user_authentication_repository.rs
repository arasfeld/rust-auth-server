use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::types::UserAuthentication;

pub struct UserAuthenticationRepositoryImpl {
    pub db: PgPool
}

#[async_trait]
pub trait UserAuthenticationRepository {
    async fn get_by_identifier(&self, service: &str, identifier: &str) -> Result<Option<UserAuthentication>, Error>;
    async fn create(
        &self,
        user_id: &Uuid,
        service: &str,
        identifier: &str,
        details: &serde_json::Value,
    ) -> Result<UserAuthentication, Error>;
}

#[async_trait]
impl UserAuthenticationRepository for UserAuthenticationRepositoryImpl {
    async fn get_by_identifier(self: &Self, service: &str, identifier: &str) -> Result<Option<UserAuthentication>, Error> {
        sqlx::query_as!(
            UserAuthentication,
            r#"
                select id, user_id
                from user_authentications
                where identifier = $1 and service = $2
                limit 1
            "#,
            identifier,
            service
        )
        .fetch_optional(&self.db)
        .await
        .map_err(Error::Sqlx)
    }

    async fn create(self: &Self, user_id: &Uuid, service: &str, identifier: &str, details: &serde_json::Value) -> Result<UserAuthentication, Error> {
        sqlx::query_as!(
            UserAuthentication,
            r#"
                insert into user_authentications (user_id, identifier, service, details)
                values ($1, $2, $3, $4)
                returning id, user_id
            "#,
            user_id,
            identifier,
            service,
            details,
        )
        .fetch_one(&self.db)
        .await
        .map_err(Error::Sqlx)
    }
}
