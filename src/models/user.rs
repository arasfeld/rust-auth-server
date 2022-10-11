use time::OffsetDateTime;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
}

pub struct UserAuthentication {
    pub id: Uuid,
    pub user_id: Uuid,
}

pub struct UserEmail {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub is_verified: bool,
}

pub struct UserSecrets {
    pub password_hash: Option<String>,
    pub failed_password_attempts: i32,
    pub first_failed_password_attempt: Option<OffsetDateTime>,
}
