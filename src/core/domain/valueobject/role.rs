use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Deserialize, Serialize, Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum Role {
    ADMIN,
    MODERATOR,
    USER,
}

impl Role {
    pub fn as_string(&self) -> String {
        match self {
            Role::ADMIN => "ADMIN".to_string(),
            Role::MODERATOR => "MODERATOR".to_string(),
            Role::USER => "USER".to_string(),
        }
    }
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match self {
            Role::ADMIN => "ADMIN",
            Role::MODERATOR => "MODERATOR",
            Role::USER => "USER",
        }
    }
}

impl From<String> for Role {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ADMIN" => Role::ADMIN,
            "MODERATOR" => Role::MODERATOR,
            "USER" => Role::USER,
            _ => panic!("Invalid role string: {}", s),
        }
    }
}
