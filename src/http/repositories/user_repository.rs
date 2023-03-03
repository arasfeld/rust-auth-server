use axum::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::http::error::{Error, ResultExt};
use crate::http::types::User;

pub struct UserRepositoryImpl {
    pub db: PgPool
}

#[async_trait]
pub trait UserRepository {
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, Error>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, Error>;
    async fn get_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    async fn create(&self, username: &str) -> Result<User, Error>;
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_by_id(self: &Self, id: &Uuid) -> Result<Option<User>, Error> {
        sqlx::query_as!(User, "select id, username from users where id = $1", id)
            .fetch_optional(&self.db)
            .await
            .map_err(Error::Sqlx)
    }

    async fn get_by_username(self: &Self, username: &str) -> Result<Option<User>, Error> {
        sqlx::query_as!(User, "select id, username from users where username = $1", username)
            .fetch_optional(&self.db)
            .await
            .map_err(Error::Sqlx)
    }

    async fn get_by_email(self: &Self, email: &str) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
                select u.id, u.username
                from user_emails ue
                    inner join users u on ue.user_id = u.id
                where ue.email = $1
                order by ue.is_verified desc, ue.created_at asc
                limit 1
            "#,
            email
        )
        .fetch_optional(&self.db)
        .await
        .map_err(Error::Sqlx)
    }

    async fn create(self: &Self, username: &str) -> Result<User, Error> {
        sqlx::query_as!(User, "insert into users (username) values ($1) returning id, username", username)
            .fetch_one(&self.db)
            .await
            .on_constraint("users_username_key", |_| {
                Error::unprocessable_entity([("username", "username taken")])
            })
    }
}
