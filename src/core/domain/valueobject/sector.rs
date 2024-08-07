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
}
