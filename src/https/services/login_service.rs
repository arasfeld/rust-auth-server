use sqlx::PgPool;

use crate::https::error::Error;
use crate::https::models::user::User;
use crate::https::repositories::{user_repository, user_secrets_repository};
use crate::https::utils::encryption::verify_password;

pub async fn login(
    db: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, Error> {
    let mut maybe_user: Option<User> = None;

    if username.contains("@") {
        // It's an email
        let user = user_repository::get_by_email(db, username).await?;
        maybe_user = Some(user)
    } else {
        // It's a username
        let user = user_repository::get_by_username(db, username).await?;
        maybe_user = Some(user)
    }

    if let Some(user) = maybe_user {
        // Load their secrets
        let user_secrets = user_secrets_repository::get_by_user_id(db, user.id).await?;

        // Have there been too many login attempts?
        if user_secrets.first_failed_password_attempt.is_some()
            && user_secrets.first_failed_password_attempt.unwrap() > time::OffsetDateTime::now_utc() - time::Duration::minutes(5)
            && user_secrets.failed_password_attempts >= 3 {
            return Err(Error::TooManyAttempts);
        }

        // Not too many login attempts, let's check the password.
        let password_hash = user_secrets.password_hash.unwrap();
        if verify_password(&password_hash, password) {
            // Excellent - they're logged in! Let's reset the attempt tracking
            user_secrets_repository::reset_login_attempts(db, user.id).await?;
            return Ok(user);
        } else {
            user_secrets_repository::add_failed_login_attempt(db, user.id).await?;
        }
    }

    Err(Error::InvalidUsernamePassword)
}
