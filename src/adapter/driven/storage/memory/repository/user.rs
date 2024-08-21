use anyhow::Error;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::adapter::driven::storage::memory::cache::MemCache;
use crate::core::domain::entity::user::User;
use crate::core::port::storage::Repo;

pub struct UserRepository {
    id_counter: Mutex<i32>,
    cache: MemCache<i32, User>,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            id_counter: Mutex::new(0),
            cache: MemCache::new(),
        }
    }
}

#[async_trait]
impl Repo<User> for UserRepository {
    async fn save(&self, user: &User) -> Result<Uuid, Error> {
        let mut counter = self.id_counter.lock().await;
        let mut owned_user = user.clone();

        *counter += 1;

        owned_user.set_id(*counter);

        self.cache.add(owned_user.id, owned_user.to_owned()).await;

        Ok(owned_user)
    }

    async fn update(&self, id_str: &str, entity: &User) -> Result<User, Error> {
        todo!()
    }

    async fn delete(&self, id_str: &str) -> Result<(), Error> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        todo!()
    }

    async fn find_by_id(&self, id_str: &str) -> Result<Option<User>, Error> {
        todo!()
    }
}
