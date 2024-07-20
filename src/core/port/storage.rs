use anyhow::Error;
use async_trait::async_trait;
use surrealdb::sql::Thing;

#[async_trait]
pub trait Storage<T> {
	async fn find_by_id(&self, id: Thing) -> Result<T, Error>;
	async fn find_all(&self) -> Result<Vec<T>, Error>;
	async fn save(&self, item: T) -> Result<Vec<T>, Error>;
	async fn update(&self, id: Thing, item: T) -> Result<T, Error>;
	async fn delete_by_id(&self, id: Thing) -> Result<(), Error>;
}
