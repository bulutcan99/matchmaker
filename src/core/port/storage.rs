use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repo<Entity>
where
    Entity: Clone + Send + Sync,
{
    async fn find_by<F, Q>(&self, filter: &F) -> Result<Option<Entity>, Error>
    where
        F: Fn(&Entity) -> Q,
        Q: PartialEq;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Entity>, Error>;
    async fn find_all(&self) -> Result<Vec<Entity>, Error>;
    async fn save(&self, entity: Entity) -> Result<Entity, Error>;
    async fn update(&self, id: Uuid, entity: Entity) -> Result<Entity, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}
