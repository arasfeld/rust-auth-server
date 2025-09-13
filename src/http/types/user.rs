use uuid::Uuid;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}
