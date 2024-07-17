use crate::core::domain::valueobject::date::DateService;
use crate::core::domain::valueobject::role::Role;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct User {
    pub id: Option<Id>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: Role,
    pub password_hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl User {
    pub fn new_user(name: String, surname: String, email: String, password_hash: String) -> Self {
        User {
            id: Some(Id::ulid()),
            name,
            surname,
            email,
            password_hash,
            role: Role::User,
            created_at: DateService::get_current_timestamp(),
            updated_at: DateService::get_current_timestamp(),
        }
    }

    pub fn new_admin(name: String, surname: String, email: String, password_hash: String) -> Self {
        User {
            id: None,
            name,
            surname,
            email,
            password_hash,
            role: Role::Admin,
            created_at: DateService::get_current_timestamp(),
            updated_at: DateService::get_current_timestamp(),
        }
    }

    pub fn new_super_admin(
        name: String,
        surname: String,
        email: String,
        password_hash: String,
    ) -> Self {
        User {
            id: None,
            name,
            surname,
            email,
            role: Role::SuperAdmin,
            password_hash,
            created_at: DateService::get_current_timestamp(),
            updated_at: DateService::get_current_timestamp(),
        }
    }

    pub fn update(
        &mut self,
        name: Option<String>,
        surname: Option<String>,
        email: Option<String>,
        password_hash: Option<String>,
    ) -> Result<(), Err()> {
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
        self.updated_at = DateService::get_curent_timestamp_utc();

        Ok(())
    }

    pub fn update_role(&mut self, role: Option<Role>) -> Result<(), Err()> {
        if let Some(role) = role {
            self.role = role;
        }

        Ok(())
    }
}
