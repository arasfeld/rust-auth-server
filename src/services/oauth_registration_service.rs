use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Error;
use crate::models::{oauth_profile::OAuthProfile, user::User};
use crate::repositories::{user_authentication_repository, user_email_repository, user_repository};
use crate::services::registration_service;
use crate::utils::username_generator;

pub async fn link_or_register_oauth_user(
    db: &PgPool,
    user_id: Option<Uuid>,
    service: &str,
    identifier: &str,
    profile: &OAuthProfile,
) -> Result<User, Error> {
    let mut maybe_user_id = user_id.clone();
    let maybe_user_authentication =
        user_authentication_repository::get_by_identifier(db, identifier, service).await?;

    if let None = maybe_user_authentication {
        if let Some(user_id) = maybe_user_id {
            // user exists, but new authentication
            maybe_user_id = Some(user_id);
            user_authentication_repository::insert(db, user_id, identifier, service).await?;
        } else {
            // look for user_email that matches
            let maybe_user_email = user_email_repository::get_by_email(db, &profile.email).await?;
            if let Some(user_email) = maybe_user_email {
                // founds user_email, add new authentication
                maybe_user_id = Some(user_email.user_id);
                user_authentication_repository::insert(db, user_email.user_id, identifier, service).await?;
            } else {
                // create new user
                let user = register_oauth_user(db, &profile.email, service, identifier).await?;

                return Ok(user);
            }
        }
    }

    // return the user
    if let Some(user_id) = maybe_user_id {
        let user = user_repository::get_by_id(db, user_id).await?;
        Ok(user)
    } else {
        Err(Error::Anyhow(anyhow::anyhow!("failed to get the user")))
    }
}

async fn register_oauth_user(
    db: &PgPool,
    email: &str,
    service: &str,
    identifier: &str,
) -> Result<User, Error> {
    let username = username_generator::generate();
    let user = registration_service::register_user(db, &username, email, None, true)
        .await
        .unwrap();

    user_authentication_repository::insert(db, user.id, identifier, service)
        .await
        .unwrap();

    Ok(user)
}
