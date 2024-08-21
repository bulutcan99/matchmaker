use anyhow::Error;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::adapter::driven::storage::memory::cache::MemCache;
use crate::core::domain::entity::user::User;
use crate::core::port::storage::Repo;

pub struct UserRepository {
    id_counter: Mutex<u64>, // `u64` gibi bir sayısal tip kullanmalısınız.
    cache: MemCache<Uuid, User>,
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

        let new_id = Uuid::new_v4();
        owned_user.set_id(new_id);

        self.cache.add(new_id, owned_user.clone()).await;

        Ok(new_id)
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
