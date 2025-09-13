use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct UserAuthentication {
  pub id: Uuid,
  pub user_id: Uuid,
}
