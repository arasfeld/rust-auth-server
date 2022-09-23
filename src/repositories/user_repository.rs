use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::User;

pub async fn insert(db: &PgPool, username: &str, name: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
            insert into users (username, name)
            values ($1, $2)
            returning id, username, name
        "#,
        username,
        name
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user)
}

