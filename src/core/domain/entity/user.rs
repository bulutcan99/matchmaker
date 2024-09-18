use anyhow::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::password::HashedPassword;
use crate::core::domain::valueobject::role::Role;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: Role,
    pub password_hash: HashedPassword,
    pub reset_token: Option<String>,
    pub reset_sent_at: Option<Timestamp>,
    pub email_verification_token: Option<String>,
    pub email_verification_sent_at: Option<Timestamp>,
    pub email_verified_at: Option<Timestamp>,
    pub blocked_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    pub fn new(name: String, surname: String, email: String, password: String, role: Role) -> Self {
        let hashed_password = HashedPassword::new(password.as_str(), &email);
        User {
            id: Some(Uuid::new_v4()),
            name,
            surname,
            email,
            role,
            password_hash: hashed_password.unwrap(),
            reset_token: None,
            reset_sent_at: None,
            email_verification_token: None,
            email_verification_sent_at: None,
            email_verified_at: None,
            blocked_at: None,
            created_at: Timestamp::now_utc(),
            updated_at: Timestamp::now_utc(),
        }
    }

    pub fn update_role(&mut self, role: Option<Role>) -> Result<(), Error> {
        if let Some(role) = role {
            self.role = role;
        }

        Ok(())
    }
}
