use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::date::DateService;
use crate::core::port::storage::Storage;

pub struct UserRepository {
	db: Arc<Pool<Postgres>>,
}

impl UserRepository {
	pub fn new(db: Arc<Pool<Postgres>>) -> Self {
		UserRepository { db }
	}
}

#[async_trait]
impl Storage<User> for UserRepository {
	async fn find_by_id(&self, id: Uuid) -> Result<User, Error> {
		let found_user = sqlx::query_as!(
            User,
            r#"
                SELECT id, name, surname, email, role, password_hash, created_at, updated_at
                FROM "user" WHERE id = $1
            "#,
            id
        )
			.fetch_one(&*self.db)
			.await?;

		Ok(found_user)
	}

	async fn find_all(&self) -> Result<Vec<User>, Error> {
		let users = sqlx::query_as!(
            User,
            r#"
                SELECT id, name, surname, email, role, password_hash, created_at, updated_at
                FROM "user"
            "#
        )
			.fetch_all(&self.db)
			.await?;

		Ok(users)
	}

	async fn save(&self, user: User) -> Result<User, Error> {
		let saved_user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO "user" (id, name, surname, email, role, password_hash, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, name, surname, email, role, password_hash, created_at, updated_at
            "#,
            user.id,
            user.name,
            user.surname,
            user.email,
            user.role.as_ref(),
            user.password_hash,
            DateService::convert_to_offset(user.created_at),
            DateService::convert_to_offset(user.updated_at),
        )
			.fetch_one(&self.db)
			.await?;

		Ok(saved_user)
	}
	async fn update(&self, id: Uuid, user: User) -> Result<User, Error> {
		let updated_user = sqlx::query_as!(
            User,
            r#"
                UPDATE "user"
                SET name = $2, surname = $3, email = $4, role = $5, password_hash = $6, updated_at = $7
                WHERE id = $1
                RETURNING id, name, surname, email, role as "role: _", password_hash, created_at, updated_at
            "#,
            id,
            user.name,
            user.surname,
            user.email,
            user.role.as_ref(),
            user.password_hash,
            DateService::convert_to_offset(user.updated_at),
        )
			.fetch_one(&self.db)
			.await?;

		Ok(updated_user)
	}

	async fn delete_by_id(&self, id: Uuid) -> Result<(), Error> {
		sqlx::query!(
            r#"
                DELETE FROM "user"
                WHERE id = $1
            "#,
            id
        )
			.execute(&self.db)
			.await?;

		Ok(())
	}
}