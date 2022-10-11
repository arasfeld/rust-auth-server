use sqlx::PgPool;

use crate::error::AppError;
use crate::models::user::User;
use crate::repositories::{user_email_repository, user_repository};

pub async fn register_user(
    db: &PgPool,
    username: &str,
    email: &str,
    password: Option<&str>,
    email_is_verified: bool,
) -> Result<User, AppError> {
    let user = user_repository::insert(db, username).await.unwrap();

    // add the user's email
    user_email_repository::insert(db, user.id, email, email_is_verified)
        .await
        .unwrap();

    // store the password

    Ok(user)
}
