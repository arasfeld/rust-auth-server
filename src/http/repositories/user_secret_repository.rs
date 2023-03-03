use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::types::UserSecret;

pub struct UserSecretRepositoryImpl {
    pub db: PgPool
}

#[async_trait]
pub trait UserSecretRepository {
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<UserSecret, Error>;
    async fn update_password_hash(&self, user_id: &Uuid, password_hash: &str) -> Result<(), Error>;
    async fn reset_login_attempts(&self, user_id: &Uuid) -> Result<(), Error>;
    async fn add_failed_login_attempt(&self, user_id: &Uuid) -> Result<(), Error>;
}

#[async_trait]
impl UserSecretRepository for UserSecretRepositoryImpl {
    async fn find_by_user_id(self: &Self, user_id: &Uuid) -> Result<UserSecret, Error> {
        sqlx::query_as!(
            UserSecret,
            r#"
                select password_hash, failed_password_attempts, first_failed_password_attempt
                from user_secrets
                where user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db)
        .await
        .map_err(Error::Sqlx)
    }
    
    async fn update_password_hash(self: &Self, user_id: &Uuid, password_hash: &str) -> Result<(), Error> {
        sqlx::query!(
            r#"
                update user_secrets
                set password_hash = $2
                where user_id = $1
            "#,
            user_id, password_hash,
        )
        .execute(&self.db)
        .await
        .map_err(Error::Sqlx)
        .and(Ok(()))
    }
    
    async fn reset_login_attempts(self: &Self, user_id: &Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
                update user_secrets
                set failed_password_attempts = 0,
                    first_failed_password_attempt = null,
                    last_login_at = now()
                where user_id = $1
            "#,
            user_id
        )
        .execute(&self.db)
        .await
        .map_err(Error::Sqlx)
        .and(Ok(()))
    }
    
    async fn add_failed_login_attempt(self: &Self, user_id: &Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
                update user_secrets
                set failed_password_attempts = (case when first_failed_password_attempt is null or first_failed_password_attempt < now() - interval '5 minutes' then 1 else failed_password_attempts + 1 end),
                    first_failed_password_attempt = (case when first_failed_password_attempt is null or first_failed_password_attempt < now() - interval '5 minutes' then now() else first_failed_password_attempt end)
                where user_id = $1
            "#,
            user_id
        )
        .execute(&self.db)
        .await
        .map_err(Error::Sqlx)
        .and(Ok(()))
    }
}
