use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Sector {
	Digital,
	Marketing,
	Advertisement,
	Software,
	AI,
	Business,
	Music,
}

impl Sector {
	pub fn to_string(&self) -> &str {
		match self {
			Sector::Digital => "Digital",
			Sector::Marketing => "Marketing",
			Sector::Advertisement => "Advertisement",
			Sector::Software => "Software",
			Sector::AI => "AI",
			Sector::Business => "Business",
			Sector::Music => "Music",
		}
	}
}