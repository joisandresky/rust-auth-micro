use chrono::Local;
use sqlx::PgPool;

use crate::domain::models::role::Role;

use super::repository::Repository;

pub struct RoleRepository {
    db_pool: PgPool
}

impl RoleRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool
        }
    }

    pub async fn get_by_name(&self, name: String) -> Result<Role, sqlx::Error> {
        let role = sqlx::query_as!(Role, "SELECT * FROM roles WHERE name = $1 AND deleted_at IS NULL", name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(role)
    }
}

#[async_trait::async_trait]
impl Repository<Role, String> for RoleRepository {
    async fn get_all(&self) -> Result<Vec<Role>, sqlx::Error> {
        let roles = sqlx::query_as!(Role, "SELECT * FROM roles WHERE deleted_at IS NULL")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(roles)
    }

    async fn get_by_id(&self, id: String) -> Result<Role, sqlx::Error> {
        let role = sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = $1 AND deleted_at IS NULL", id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(role)
    }

    async fn create(&self, entity: Role) -> Result<Role, sqlx::Error> {
        let role = sqlx::query_as!(Role, "INSERT INTO roles (id, name, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *", entity.id, entity.name, entity.description, entity.created_at, entity.updated_at)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(role)
    }

    async fn update(&self, entity: Role) -> Result<Role, sqlx::Error> {
        let now = Local::now().naive_local();
        let role = sqlx::query_as!(Role, "UPDATE roles SET name = $1, description = $2, updated_at = $3 WHERE id = $4 RETURNING *", entity.name, entity.description, now, entity.id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(role)
    }

    async fn delete_by_id(&self, id: String) -> Result<(), sqlx::Error> {
        let now = Local::now().naive_local();
        sqlx::query!("UPDATE roles SET deleted_at = $1 WHERE id = $2", now, id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }
}