use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Sector{
	Digital,
	Marketing,
	Advertisement,
}