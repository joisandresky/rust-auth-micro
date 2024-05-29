use chrono::Local;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::models::user::User;

use super::repository::Repository;


pub struct UserRepository {
    db_pool: PgPool
}

impl UserRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool
        }
    }

    pub async fn is_email_exist(&self, email: &String) -> Result<bool, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL", email)
           .fetch_optional(&self.db_pool)
           .await?;

        Ok(user.is_some())
    }

    pub async fn tx_create(&self, tx: &mut Transaction<'_, Postgres>, entity: User) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "INSERT INTO users (id, email, password, email_verified_at, last_login, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING
        * ", entity.id, entity.email, entity.password, entity.email_verified_at, entity.last_login, entity.is_active, entity.created_at, entity.updated_at)
            .fetch_one(&mut **tx)
            .await?;

        Ok(user)
    }
}

#[async_trait::async_trait]
impl Repository<User, String> for UserRepository {
    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(User, "SELECT * FROM users WHERE deleted_at IS NULL")
           .fetch_all(&self.db_pool)
           .await?;

        Ok(users)
    }

    async fn get_by_id(&self, id: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL", id)
           .fetch_one(&self.db_pool)
           .await?;

        Ok(user)
    }

    async fn create(&self, entity: User) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "INSERT INTO users (id, email, password, email_verified_at, last_login, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING
        * ", entity.id, entity.email, entity.password, entity.email_verified_at, entity.last_login, entity.is_active, entity.created_at, entity.updated_at)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user)
    }

    async fn update(&self, entity: User) -> Result<User, sqlx::Error> {
        let now = Local::now().naive_local();
        let user = sqlx::query_as!(User, "UPDATE users SET email = $1, password = $2, email_verified_at = $3, last_login = $4, is_active = $5, updated_at = $6 WHERE id = $7
        RETURNING * ", entity.email, entity.password, entity.email_verified_at, entity.last_login, entity.is_active, now, entity.id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user)
    }

    async fn delete_by_id(&self, id: String) -> Result<(), sqlx::Error> {
        let now = Local::now().naive_local();
        let _ = sqlx::query!("UPDATE users SET deleted_at = $1 WHERE id = $2", now, id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}
