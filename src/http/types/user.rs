use uuid::Uuid;

#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
