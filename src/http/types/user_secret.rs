use time::OffsetDateTime;

pub struct UserSecret {
    pub password_hash: Option<String>,
    pub failed_password_attempts: i32,
    pub first_failed_password_attempt: Option<OffsetDateTime>,
}
