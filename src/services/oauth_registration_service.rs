use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{oauth_profile::OAuthProfile, user::User};
use crate::repositories::{user_authentication_repository, user_email_repository};
use crate::services::registration_service;
use crate::utils::username_generator;

pub async fn link_or_register_oauth_user(
    db: &PgPool,
    user_id: Option<Uuid>,
    service: &str,
    identifier: &str,
    profile: &OAuthProfile,
) -> Result<bool, AppError> {
    let maybe_user_authentication =
        user_authentication_repository::get_by_identifier(db, identifier, service)
            .await
            .unwrap();

    if let None = maybe_user_authentication {
        if let Some(user_id) = user_id {
            // user exists, but new authentication
            user_authentication_repository::insert(db, user_id, identifier, service)
                .await
                .unwrap();
        } else {
            // look for user_email that matches
            let maybe_user_email = user_email_repository::get_by_email(db, &profile.email)
                .await
                .unwrap();
            if let Some(user_email) = maybe_user_email {
                // founds user_email, add new authentication
                user_authentication_repository::insert(db, user_email.user_id, identifier, service)
                    .await
                    .unwrap();
            } else {
                // create new user
                register_oauth_user(db, &profile.email, &profile.name, service, identifier)
                    .await
                    .unwrap();
            }
        }
    }

    Ok(true)
}

async fn register_oauth_user(
    db: &PgPool,
    email: &str,
    name: &str,
    service: &str,
    identifier: &str,
) -> Result<User, AppError> {
    let username = username_generator::generate();
    let user = registration_service::register_user(db, &username, email, name, None, true)
        .await
        .unwrap();

    user_authentication_repository::insert(db, user.id, identifier, service)
        .await
        .unwrap();

    Ok(user)
}
