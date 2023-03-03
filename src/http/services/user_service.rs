use axum::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::http::error::Error;
use crate::http::repositories::UserRepository;
use crate::http::types::User;

pub struct UserServiceImpl<A: UserRepository> {
    pub user_repository: Arc<A>,
}

#[async_trait]
pub trait UserService {
    async fn get_by_id(self: &Self, id: &Uuid) -> Result<Option<User>, Error>;
}

pub type DynUserService = Arc<dyn UserService + Send + Sync>;

#[async_trait]
impl <A> UserService for UserServiceImpl<A>
    where A: UserRepository + Sync + Send {
    
    async fn get_by_id(self: &Self, id: &Uuid) -> Result<Option<User>, Error> {
        self.user_repository.get_by_id(id).await
    }
}
