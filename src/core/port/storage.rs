use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Repo<Entity>: Send + Sync
where
    Entity: Clone + Send + Sync,
{
    async fn save(&self, entity: &Entity) -> Result<Entity, Error>;
    async fn update(&self, id_str: &str, entity: &Entity) -> Result<Entity, Error>;
    async fn delete(&self, id_str: &str) -> Result<(), Error>;
    async fn find_all(&self) -> Result<Vec<Entity>, Error>;
    async fn find_by_id(&self, id_str: &str) -> Result<Option<Entity>, Error>;
    async fn find_by<F, Q>(&self, filter: &F) -> Result<Option<Entity>, Error>
    where
        F: Fn(&Entity) -> Q,
        Q: PartialEq;
}
