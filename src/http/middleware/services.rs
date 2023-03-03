use sqlx::PgPool;
use std::sync::Arc;

use crate::http::{
    repositories::{UserAuthenticationRepositoryImpl, UserEmailRepositoryImpl, UserRepositoryImpl, UserSecretRepositoryImpl},
    services::{DynLoginService, DynOAuthRegistrationService, DynRegistrationService, DynUserService,
        LoginServiceImpl, OAuthRegistrationServiceImpl, RegistrationServiceImpl, UserServiceImpl},
};

pub struct Services {
    pub login_service: DynLoginService,
    pub oauth_registration_service: DynOAuthRegistrationService,
    pub registration_service: DynRegistrationService,
    pub user_service: DynUserService,
}

pub fn build(db: PgPool) -> Services {
    let user_repository = Arc::new(UserRepositoryImpl { db: db.clone() });
    let user_authentication_repository = Arc::new(UserAuthenticationRepositoryImpl { db: db.clone() });
    let user_email_repository = Arc::new(UserEmailRepositoryImpl { db: db.clone() });
    let user_secret_repository = Arc::new(UserSecretRepositoryImpl { db: db.clone() });

    Services {
        login_service: Arc::new(
            LoginServiceImpl {
                user_repository: user_repository.clone(),
                user_secret_repository: user_secret_repository.clone(),
            }
        ),
        oauth_registration_service: Arc::new(
            OAuthRegistrationServiceImpl {
                user_repository: user_repository.clone(),
                user_authentication_repository: user_authentication_repository.clone(),
                user_email_repository: user_email_repository.clone(),
            }
        ),
        registration_service: Arc::new(
            RegistrationServiceImpl {
                user_repository: user_repository.clone(),
                user_secret_repository: user_secret_repository.clone(),
                user_email_repository: user_email_repository.clone(),
            }
        ),
        user_service: Arc::new(
            UserServiceImpl {
                user_repository: user_repository.clone(),
            }
        ),
    }
}
