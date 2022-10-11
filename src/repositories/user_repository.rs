use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::User;

pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
            select id, username
            from users
            where id = $1
        "#,
        id
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user)
}

pub async fn get_by_username(db: &PgPool, username: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
            select id, username
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user)
}

pub async fn get_by_email(db: &PgPool, email: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
            select users.id, users.username
            from users
                inner join user_emails on users.id = user_emails.user_id
            where user_emails.email = $1
            order by
                user_emails.is_verified desc,
                user_emails.created_at asc
            limit 1
        "#,
        email
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user)
}

pub async fn insert(db: &PgPool, username: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
            insert into users (username)
            values ($1)
            returning id, username
        "#,
        username,
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user)
}
