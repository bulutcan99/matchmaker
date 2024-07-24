use anyhow::Error;

pub trait CompanyManagement: Send + Sync {
	async fn register(&self, input: &None) -> Result<None, Error>;
	async fn get_profile(&self, input: &None) -> Result<None, Error>;
}

