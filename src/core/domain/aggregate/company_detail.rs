use crate::core::domain::entity::user::User;
use crate::core::domain::entity::company::Company;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompanyDetail{
    pub company: Company,
    pub users: Vec<User>,
}