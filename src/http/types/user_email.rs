use uuid::Uuid;

pub struct UserEmail {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub is_verified: bool,
}
