use std::sync::Arc;

use anyhow::{anyhow, Error};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::date::Timestamp;
use crate::core::port::user::UserRepo;

#[derive(Debug, Clone)]
pub struct UserRepository {
    db: Arc<Pool<Postgres>>,
}

impl UserRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        UserRepository { db }
    }
}
#[async_trait]
impl UserRepo for UserRepository {
    async fn save(&self, user: &User) -> Result<Uuid, Error> {
        let saved_user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO "user" (id, name, surname, email, role, password_hash, reset_token, reset_sent_at, email_verification_token, email_verification_sent_at, email_verified_at, blocked_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id
            "#,
            user.id,
            user.name,
            user.surname,
            user.email,
            user.role.as_string(),
            user.password_hash.as_string(),
            user.reset_token,
            user.reset_sent_at,
            user.email_verification_token,
            user.email_verification_sent_at,
            user.email_verified_at,
            user.blocked_at,
             Timestamp::now_utc().convert_to_offset(), // Assume this method is correct for your context
             Timestamp::now_utc().convert_to_offset()  // Assume this method is correct for your context
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(saved_user_id)
    }

    async fn update(&self, id_str: &str, user: &User) -> Result<User, Error> {
        let id = Uuid::parse_str(id_str)?;
        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE "user"
            SET
                name = COALESCE($2, name),
                surname = COALESCE($3, surname),
                email = COALESCE($4, email),
                role = COALESCE($5, role),
                password_hash = COALESCE($6, password_hash),
                reset_token = COALESCE($7, reset_token),
                reset_sent_at = COALESCE($8, reset_sent_at),
                email_verification_token = COALESCE($9, email_verification_token),
                email_verification_sent_at = COALESCE($10, email_verification_sent_at),
                email_verified_at = COALESCE($11, email_verified_at),
                blocked_at = COALESCE($12, blocked_at),
                updated_at = COALESCE($13, updated_at)
            WHERE id = $1
            RETURNING id, name, surname, email, role as "role: _", password_hash, reset_token, reset_sent_at, email_verification_token, email_verification_sent_at, email_verified_at, blocked_at, created_at, updated_at
            "#,
            id,
            user.name,
            user.surname,
            user.email,
            user.role.as_string(),
            user.password_hash.as_string(),
            user.reset_token,
            user.reset_sent_at,
            user.email_verification_token,
            user.email_verification_sent_at,
            user.email_verified_at,
            user.blocked_at,
             Timestamp::now_utc().convert_to_offset() // Assume this method is correct for your context
        )
        .fetch_one(&*self.db)
        .await?;

        Ok(updated_user)
    }

    async fn delete(&self, id_str: &str) -> Result<(), Error> {
        let id = Uuid::parse_str(id_str)?;

        sqlx::query!(
            r#"
            DELETE FROM "user"
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.db)
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, name, surname, email, role, password_hash, reset_token, reset_sent_at, email_verification_token, email_verification_sent_at, email_verified_at, blocked_at, created_at, updated_at
            FROM "user"
            "#
        )
        .fetch_all(&*self.db)
        .await
        .map_err(|err| anyhow!("Error getting users: {}", err))
    }

    async fn find_by_id(&self, id_str: &str) -> Result<Option<User>, Error> {
        let id = Uuid::parse_str(id_str)?;
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, surname, email, role, password_hash, reset_token, reset_sent_at, email_verification_token, email_verification_sent_at, email_verified_at, blocked_at, created_at, updated_at
            FROM "user"
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.db)
        .await
        .map_err(|err| anyhow!("Error getting user by id: {}", err))?;

        Ok(row)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, surname, email, role, password_hash, reset_token, reset_sent_at, email_verification_token, email_verification_sent_at, email_verified_at, blocked_at, created_at, updated_at
            FROM "user"
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&*self.db)
        .await
        .map_err(|err| anyhow!("Error getting user by email: {}", err))?;

        Ok(user)
    }
}
