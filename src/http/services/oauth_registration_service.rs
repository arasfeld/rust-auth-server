use axum::async_trait;
use std::sync::Arc;

use crate::http::error::Error;
use crate::http::repositories::{UserAuthenticationRepository, UserEmailRepository, UserRepository};
use crate::http::types::User;
use crate::http::utils::username_generator;

pub struct OAuthRegistrationServiceImpl<A: UserRepository, B: UserAuthenticationRepository, C: UserEmailRepository> {
    pub user_repository: Arc<A>,
    pub user_authentication_repository: Arc<B>,
    pub user_email_repository: Arc<C>,
}

#[async_trait]
pub trait OAuthRegistrationService {
    async fn link_or_register_oauth_user(
        self: &Self,
        email: &str,
        service: &str,
        identifier: &str,
        details: &serde_json::Value,
    ) -> Result<User, Error>;
}

pub type DynOAuthRegistrationService = Arc<dyn OAuthRegistrationService + Send + Sync>;

#[async_trait]
impl <A, B, C> OAuthRegistrationService for OAuthRegistrationServiceImpl<A, B, C>
    where A: UserRepository + Sync + Send,
          B: UserAuthenticationRepository + Sync + Send,
          C: UserEmailRepository + Sync + Send {

    async fn link_or_register_oauth_user(
        self: &Self,
        email: &str,
        service: &str,
        identifier: &str,
        details: &serde_json::Value,
    ) -> Result<User, Error> {
        let user: User = match self.user_authentication_repository.get_by_identifier(service, identifier).await? {
            Some(user_authentication) => {
                // user has already authenticated with this oauth provider
                self.user_repository.get_by_id(&user_authentication.user_id).await?.unwrap()
            },
            None => {
                // look for user_email that matches
                if let Some(user_email) = self.user_email_repository.get_by_email(email).await? {
                    // found user_email, add new authentication
                    self.user_authentication_repository.create(&user_email.user_id, identifier, service, details).await?;
                    self.user_repository.get_by_id(&user_email.user_id).await?.unwrap()
                } else {
                    // create new user
                    let username = username_generator::generate();
                    let user = self.user_repository.create(&username).await?;
                    self.user_email_repository.create(&user.id, email, true).await?;
                    self.user_authentication_repository.create(&user.id, identifier, service, details).await?;
                    user
                }
            },
        };
        return Ok(user);
    }
}
