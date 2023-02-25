use sqlx::PgPool;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::models::user::UserEmail;

pub async fn get_by_email(db: &PgPool, email: &str) -> Result<Option<UserEmail>, Error> {
    let user_email = sqlx::query_as!(
        UserEmail,
        r#"
            select id, user_id, email, is_verified
            from user_emails
            where email = $1
            order by is_verified desc, created_at asc
            limit 1
        "#,
        email,
    )
    .fetch_optional(db)
    .await?;

    Ok(user_email)
}

pub async fn insert(
    db: &PgPool,
    user_id: Uuid,
    email: &str,
    is_verified: bool,
) -> Result<UserEmail, Error> {
    let user_email = sqlx::query_as!(
        UserEmail,
        r#"
            insert into user_emails (user_id, email, is_verified)
            values ($1, lower($2), $3)
            returning id, user_id, email, is_verified
        "#,
        user_id,
        email,
        is_verified
    )
    .fetch_one(db)
    .await?;

    Ok(user_email)
}
