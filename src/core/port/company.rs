use anyhow::Error;
use async_trait::async_trait;

use crate::core::domain::entity::company::Company;

#[async_trait]
pub trait CompanyRepo: Send + Sync {
    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, Error>;
}

// #[async_trait]
// pub trait CompanyManagement: Send + Sync {
//     async fn register(&self, input: &None) -> Result<None, Error>;
//     async fn get_profile(&self, input: &None) -> Result<None, Error>;
// }
