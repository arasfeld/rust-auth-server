use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: Option<String>,
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
