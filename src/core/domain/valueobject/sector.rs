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
    pub fn to_string(&self) -> String {
        match self {
            Sector::Digital => "Digital".to_owned(),
            Sector::Marketing => "Marketing".to_owned(),
            Sector::Advertisement => "Advertisement".to_owned(),
            Sector::Software => "Software".to_owned(),
            Sector::AI => "AI".to_owned(),
            Sector::Business => "Business".to_owned(),
            Sector::Music => "Music".to_owned(),
        }
    }

    pub fn from_string(domain: &str) -> Option<Self> {
        match domain {
            "Digital" => Some(Sector::Digital),
            "Marketing" => Some(Sector::Marketing),
            "Advertisement" => Some(Sector::Advertisement),
            "Software" => Some(Sector::Software),
            "AI" => Some(Sector::AI),
            "Business" => Some(Sector::Business),
            "Music" => Some(Sector::Music),
            _ => None,
        }
    }
}
