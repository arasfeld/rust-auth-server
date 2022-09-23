use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::user::UserAuthentication;

pub async fn get_by_identifier(
    db: &PgPool,
    identifier: &str,
    service: &str,
) -> Result<Option<UserAuthentication>, AppError> {
    let user_authentication = sqlx::query_as!(
        UserAuthentication,
        r#"
            select id, user_id
            from user_authentications
            where identifier = $1 and service = $2
        "#,
        identifier,
        service
    )
    .fetch_optional(db)
    .await
    .unwrap();

    Ok(user_authentication)
}

pub async fn insert(
    db: &PgPool,
    user_id: Uuid,
    identifier: &str,
    service: &str,
) -> Result<UserAuthentication, AppError> {
    let user_authentication = sqlx::query_as!(
        UserAuthentication,
        r#"
            insert into user_authentications (user_id, identifier, service)
            values ($1, $2, $3)
            returning id, user_id
        "#,
        user_id,
        identifier,
        service,
    )
    .fetch_one(db)
    .await
    .unwrap();

    Ok(user_authentication)
}
