use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;

use crate::core::domain::entity::company::Company;

#[async_trait]
pub trait CompanyRepo: Send + Sync {
    async fn save(&self, entity: &Company) -> Result<Uuid, Error>;
    async fn update(&self, id_str: &str, entity: &Company) -> Result<Company, Error>;
    async fn delete(&self, id_str: &str) -> Result<(), Error>;
    async fn find_all(&self) -> Result<Vec<Company>, Error>;
    async fn find_by_id(&self, id_str: &str) -> Result<Option<Company>, Error>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, Error>;
}

// #[async_trait]
// pub trait CompanyManagement: Send + Sync {
//     async fn register(&self, input: &None) -> Result<None, Error>;
//     async fn get_profile(&self, input: &None) -> Result<None, Error>;
// }
