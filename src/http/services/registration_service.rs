use axum::async_trait;
use std::sync::Arc;

use crate::http::error::Error;
use crate::http::repositories::{UserEmailRepository, UserRepository, UserSecretRepository};
use crate::http::types::User;
use crate::http::utils::encryption;

pub struct RegistrationServiceImpl<A: UserRepository, B: UserSecretRepository, C: UserEmailRepository> {
    pub user_repository: Arc<A>,
    pub user_secret_repository: Arc<B>,
    pub user_email_repository: Arc<C>,
}

#[async_trait]
pub trait RegistrationService {
    async fn register_user(
        self: &Self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, Error>;
}

pub type DynRegistrationService = Arc<dyn RegistrationService + Send + Sync>;

#[async_trait]
impl <A, B, C> RegistrationService for RegistrationServiceImpl<A, B, C>
    where A: UserRepository + Sync + Send,
          B: UserSecretRepository + Sync + Send,
          C: UserEmailRepository + Sync + Send {

    async fn register_user(
        self: &Self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, Error> {
        let user = self.user_repository.create(username).await?;
    
        // add the user's email
        self.user_email_repository.create(&user.id, email, false).await?;
    
        // store the password
        let password_hash = encryption::hash_password(password);
        self.user_secret_repository.update_password_hash(&user.id, &password_hash).await?;
    
        Ok(user)
    }
}
