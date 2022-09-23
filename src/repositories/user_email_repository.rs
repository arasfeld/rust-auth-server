use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::UserEmail;

pub async fn get_by_email(db: &PgPool, email: &str) -> Result<Option<UserEmail>, AppError> {
    let user_email = sqlx::query_as!(
        UserEmail,
        r#"
            select id, user_id, email, is_verified
            from user_emails
            where email = $1
        "#,
        email,
    )
    .fetch_optional(db)
    .await
    .unwrap();

    Ok(user_email)
}

pub async fn insert(
    db: &PgPool,
    user_id: Uuid,
    email: &str,
    is_verified: bool,
) -> Result<UserEmail, AppError> {
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
    .await
    .unwrap();

    Ok(user_email)
}
