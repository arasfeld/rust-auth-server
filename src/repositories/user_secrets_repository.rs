use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::UserSecrets;

pub async fn get_by_user_id(db: &PgPool, user_id: Uuid) -> Result<UserSecrets, AppError> {
    let user = sqlx::query_as!(
        UserSecrets,
        r#"
            select password_hash
            from user_secrets
            where user_id = $1
        "#,
        user_id
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user)
}

pub async fn update_password_hash(db: &PgPool, user_id: Uuid, password_hash: &str) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            update user_secrets
            set password_hash = $2
            where user_id = $1
        "#,
        user_id, password_hash,
    )
    .execute(db)
    .await
    .unwrap();

    Ok(())
}
