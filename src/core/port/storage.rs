use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Storage<T>: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> Result<T, Error>;
	async fn find_all(&self) -> Result<Vec<T>, Error>;
	async fn save(&self, item: T) -> Result<Vec<T>, Error>;
	async fn update(&self, id: Uuid, item: T) -> Result<T, Error>;
	async fn delete_by_id(&self, id: Uuid) -> Result<(), Error>;
}
