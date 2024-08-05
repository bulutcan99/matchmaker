use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Role {
    Admin,
    Moderator,
    User,
}

impl Role {
    pub fn as_string(&self) -> String {
        match self {
            Role::Admin => "Admin".to_string(),
            Role::Moderator => "Moderator".to_string(),
            Role::User => "User".to_string(),
        }
    }
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match self {
            Role::Admin => "Admin",
            Role::Moderator => "Moderator",
            Role::User => "User",
        }
    }
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Admin" => Role::Admin,
            "Moderator" => Role::Moderator,
            "User" => Role::User,
            _ => panic!("Invalid role string: {}", s),
        }
    }
}
