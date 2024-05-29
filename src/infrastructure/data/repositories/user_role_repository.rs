use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::models::user_role::UserRole;

use super::repository::Repository;


pub struct UserRoleRepository {
    db_pool: PgPool,
}

impl UserRoleRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool,
        }
    }

    pub async fn is_exist(&self, user_id: &String, role_id: &String) -> Result<bool, sqlx::Error> {
        let user_role = sqlx::query_as!(UserRole, "SELECT * FROM user_roles WHERE user_id = $1 AND role_id = $2", user_id, role_id)
            .fetch_optional(&self.db_pool)
            .await?;

        Ok(user_role.is_some())
    }

    pub async fn tx_create(&self, tx: &mut Transaction<'_, Postgres>, entity: UserRole) -> Result<UserRole, sqlx::Error> {
        let user_role = sqlx::query_as!(UserRole, "INSERT INTO user_roles (id, user_id, role_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *", entity.id, entity.user_id, entity.role_id, entity.created_at, entity.updated_at)
            .fetch_one(&mut **tx)
            .await?;

        Ok(user_role)
    }

    pub async fn delete_user_role(&self, user_id: String, role_id: String) -> Result<UserRole, sqlx::Error> {
        let user_role = sqlx::query_as!(UserRole, "DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2 RETURNING *", user_id, role_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user_role)
    }
}

#[async_trait::async_trait]
impl Repository<UserRole, String> for UserRoleRepository {
    // only implement create and update, other else just set todo!()
    
    async fn get_all(&self) -> Result<Vec<UserRole>, sqlx::Error> {
        todo!()
    }

    async fn create(&self, entity: UserRole) -> Result<UserRole, sqlx::Error> {
        let user_role = sqlx::query_as!(UserRole, "INSERT INTO user_roles (id, user_id, role_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *", entity.id, entity.user_id, entity.role_id, entity.created_at, entity.updated_at)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user_role)
    }

    async fn update(&self, entity: UserRole) -> Result<UserRole, sqlx::Error> {
        let user_role = sqlx::query_as!(UserRole, "UPDATE user_roles SET user_id = $1, role_id = $2 WHERE id = $3 RETURNING *", entity.user_id, entity.role_id, entity.id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(user_role)
    }

    async fn get_by_id(&self, _id: String) -> Result<UserRole, sqlx::Error> {
        todo!()
    }

    async fn delete_by_id(&self, _id: String) -> Result<(), sqlx::Error> {
        todo!()
    }
}