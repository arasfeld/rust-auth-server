use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::types::UserEmail;

pub struct UserEmailRepositoryImpl {
    pub db: PgPool
}

#[async_trait]
pub trait UserEmailRepository {
    async fn get_by_email(&self, email: &str) -> Result<Option<UserEmail>, Error>;
    async fn create(&self, user_id: &Uuid, email: &str, is_verified: bool) -> Result<UserEmail, Error>;
}

#[async_trait]
impl UserEmailRepository for UserEmailRepositoryImpl {
    async fn get_by_email(self: &Self, email: &str) -> Result<Option<UserEmail>, Error> {
        sqlx::query_as::<_, UserEmail>(
            r#"
                select id, user_id, email, is_verified
                from user_emails
                where email = $1
                order by is_verified desc, created_at asc
                limit 1
            "#
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await
        .map_err(Error::Sqlx)
    }

    async fn create(self: &Self, user_id: &Uuid, email: &str, is_verified: bool) -> Result<UserEmail, Error> {
        sqlx::query_as::<_, UserEmail>(
            r#"
                insert into user_emails (user_id, email, is_verified)
                values ($1, lower($2), $3)
                returning id, user_id, email, is_verified
            "#
        )
        .bind(user_id)
        .bind(email)
        .bind(is_verified)
        .fetch_one(&self.db)
        .await
        .map_err(Error::Sqlx)
    }
}
