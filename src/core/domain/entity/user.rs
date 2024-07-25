use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::role::Role;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: Role,
    pub password_hash: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    pub fn new(
        name: String,
        surname: String,
        email: String,
        password_hash: String,
        role: Role,
    ) -> Self {
        User {
            id,
            name,
            surname,
            email,
            role,
            password_hash,
            created_at: Timestamp::now_utc(),
            updated_at: Timestamp::now_utc(),
        }
    }

    pub fn update(
        &mut self,
        name: Option<String>,
        surname: Option<String>,
        email: Option<String>,
        password_hash: Option<String>,
    ) -> Result<(), Error> {
        if let Some(password_hash) = password_hash {
            self.password_hash = password_hash;
        }
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(surname) = surname {
            self.surname = surname;
        }
        if let Some(email) = email {
            self.email = email;
        }
        self.updated_at = Timestamp::now_utc();

        Ok(())
    }

    pub fn update_role(&mut self, role: Option<Role>) -> Result<(), Error> {
        if let Some(role) = role {
            self.role = role;
        }

        Ok(())
    }

    pub fn get_role_string(&self) -> &str {
        self.role.as_ref()
    }
}
