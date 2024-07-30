use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::date::Timestamp;
use crate::core::port::storage::Repo;
use crate::core::port::user::UserRepo;

pub struct UserRepository {
    db: Arc<Pool<Postgres>>,
}

impl UserRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        UserRepository { db }
    }
}

#[async_trait]
impl Repo<User> for UserRepository {
    async fn find_by<F, Q>(&self, filter: &F) -> Result<Option<User>, Error>
    where
        F: Fn(&User) -> Q,
        Q: PartialEq,
    {
        let users = self.find_all().await?;
        let filtered_user = users
            .into_iter()
            .find(|user| filter(user) == filter(&users[0]));

        Ok(filtered_user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, surname, email, role, password_hash, created_at, updated_at
        FROM "user" WHERE id = $1
    "#,
            id
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|err| Error::from("Error from getting user by id"))
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, name, surname, email, role, password_hash, created_at, updated_at
            FROM "user"
        "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|err| Error::from("Error from getting user by id"))
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
            user.password_hash.as_str(),
            Timestamp::now_utc().convert_to_offset(),
            Timestamp::now_utc().convert_to_offset(),

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
            SET
                name = COALESCE($2, name),
                surname = COALESCE($3, surname),
                email = COALESCE($4, email),
                role = COALESCE($5, role),
                password_hash = COALESCE($6, password_hash),
                updated_at = COALESCE($7, updated_at)
            WHERE id = $1
            RETURNING id, name, surname, email, role as "role: _", password_hash, created_at, updated_at
        "#,
        id,
        user.name.as_deref(),
        user.surname.as_deref(),
        user.email.as_deref(),
        user.role.as_ref(),
        user.password_hash.as_deref(),
        Timestamp::now_utc().convert_to_offset(),
    )
    .fetch_one(&self.db)
    .await?;

        Ok(updated_user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
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

#[async_trait]
impl UserRepo for UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, name, surname, email, role, password_hash, created_at, updated_at
            FROM "user" WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|err| Error::from("Error from getting user by email"))
    }
}
