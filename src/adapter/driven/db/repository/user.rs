use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::user::User;
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
                SELECT * FROM users WHERE id = $1
            "#,
            id
        )
			.fetch_optional(&*self.db)
			.await?;

		found_user.ok_or_else(|| Error::msg("User not found"))
	}

	async fn find_all(&self) -> Result<Vec<User>, Error> {
		let users = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
            "#
        )
			.fetch_all(&*self.db)
			.await?;

		Ok(users)
	}

	async fn save(&self, user: User) -> Result<User, Error> {
		let saved_user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (id, name, email) VALUES ($1, $2, $3)
                RETURNING id, name, email
            "#,
            user.id,
            user.name,
            user.email
        )
			.fetch_one(&*self.db)
			.await?;

		Ok(saved_user)
	}

	async fn update(&self, id: Uuid, user: User) -> Result<User, Error> {
		let updated_user = sqlx::query_as!(
            User,
            r#"
                UPDATE users SET name = $2, email = $3 WHERE id = $1
                RETURNING id, name, email
            "#,
            id,
            user.name,
            user.email
        )
			.fetch_one(&*self.db)
			.await?;

		Ok(updated_user)
	}

	async fn delete_by_id(&self, id: Uuid) -> Result<(), Error> {
		sqlx::query!(
            r#"
                DELETE FROM users WHERE id = $1
            "#,
            id
        )
			.execute(&*self.db)
			.await?;

		Ok(())
	}
}