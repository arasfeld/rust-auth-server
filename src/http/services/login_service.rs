use axum::async_trait;
use std::sync::Arc;
use chrono::{Utc, Duration};

use crate::http::error::Error;
use crate::http::repositories::{UserRepository, UserSecretRepository};
use crate::http::types::User;
use crate::http::utils::encryption::verify_password;

pub struct LoginServiceImpl<A: UserRepository, B: UserSecretRepository> {
    pub user_repository: Arc<A>,
    pub user_secret_repository: Arc<B>,
}

#[async_trait]
pub trait LoginService {
    async fn login(self: &Self, username: &str, password: &str) -> Result<User, Error>;
}

pub type DynLoginService = Arc<dyn LoginService + Send + Sync>;

#[async_trait]
impl <A, B> LoginService for LoginServiceImpl<A, B>
    where A: UserRepository + Sync + Send,
          B: UserSecretRepository + Sync + Send {
    
    async fn login(self: &Self, username: &str, password: &str) -> Result<User, Error> {
        let maybe_user = match username.contains("@") {
            true => self.user_repository.get_by_email(username).await?,
            false => self.user_repository.get_by_username(username).await?
        };
    
        if let Some(user) = maybe_user {
            // Load their secrets
            let user_secrets = self.user_secret_repository.find_by_user_id(&user.id).await?;
    
            // Have there been too many login attempts?
            if user_secrets.first_failed_password_attempt.is_some()
                && user_secrets.first_failed_password_attempt.unwrap() > Utc::now() - Duration::minutes(5)
                && user_secrets.failed_password_attempts >= 3 {
                return Err(Error::TooManyAttempts);
            }
    
            // Not too many login attempts, let's check the password.
            let password_hash = user_secrets.password_hash.unwrap();
            if verify_password(&password_hash, password) {
                // Excellent - they're logged in! Let's reset the attempt tracking
                self.user_secret_repository.reset_login_attempts(&user.id).await?;
                return Ok(user);
            } else {
                self.user_secret_repository.add_failed_login_attempt(&user.id).await?;
            }
        }
    
        Err(Error::InvalidUsernamePassword)
    }
}
