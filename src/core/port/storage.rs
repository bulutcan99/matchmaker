use anyhow::Error;
use async_trait::async_trait;
use surrealdb::sql::Id;

#[async_trait]
pub trait Storage<T> {
	async fn find_by_id(&self, id: Id) -> Option<T>;
	async fn find_all(&self) -> Result<Vec<T>, Error>;
	async fn save(&self, item: &T) -> Result<(), Error>;
	async fn update(&self, item: &T) -> Result<T, Error>;
	async fn delete_by_id(&self, id: Id) -> Result<(), Error>;
}
