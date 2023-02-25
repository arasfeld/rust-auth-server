use sqlx::PgPool;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::models::user::UserSecrets;

pub async fn get_by_user_id(db: &PgPool, user_id: Uuid) -> Result<UserSecrets, Error> {
    let user = sqlx::query_as!(
        UserSecrets,
        r#"
            select password_hash, failed_password_attempts, first_failed_password_attempt
            from user_secrets
            where user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}

pub async fn update_password_hash(db: &PgPool, user_id: Uuid, password_hash: &str) -> Result<(), Error> {
    sqlx::query!(
        r#"
            update user_secrets
            set password_hash = $2
            where user_id = $1
        "#,
        user_id, password_hash,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn reset_login_attempts(db: &PgPool, user_id: Uuid) -> Result<(), Error> {
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
    .execute(db)
    .await?;

    Ok(())
}

pub async fn add_failed_login_attempt(db: &PgPool, user_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        r#"
            update user_secrets
            set failed_password_attempts = (case when first_failed_password_attempt is null or first_failed_password_attempt < now() - interval '5 minutes' then 1 else failed_password_attempts + 1 end),
                first_failed_password_attempt = (case when first_failed_password_attempt is null or first_failed_password_attempt < now() - interval '5 minutes' then now() else first_failed_password_attempt end)
            where user_id = $1
        "#,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}
