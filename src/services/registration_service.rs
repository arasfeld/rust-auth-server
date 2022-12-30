use sqlx::PgPool;

use crate::error::Error;
use crate::models::user::User;
use crate::repositories::{user_email_repository, user_repository, user_secrets_repository};
use crate::utils::encryption;

pub async fn register_user(
    db: &PgPool,
    username: &str,
    email: &str,
    maybe_password: Option<&str>,
    email_is_verified: bool,
) -> Result<User, Error> {
    let user = user_repository::insert(db, username).await?;

    // add the user's email
    user_email_repository::insert(db, user.id, email, email_is_verified).await?;

    // store the password
    if let Some(password) = maybe_password {
        let password_hash = encryption::hash_password(password);
        user_secrets_repository::update_password_hash(db, user.id, &password_hash).await?;
    }

    Ok(user)
}
