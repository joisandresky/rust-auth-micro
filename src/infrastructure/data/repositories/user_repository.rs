use chrono::Local;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::models::{role::Role, user::User, user_role::UserWithRoles};

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

    pub async fn get_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL", email)
           .fetch_one(&self.db_pool)
           .await?;

        Ok(user)
    }

    pub async fn get_user_by_id_with_roles(&self, id: String) -> Result<UserWithRoles, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                u.id as user_id, u.email, u.password, u.email_verified_at, u.last_login, u.is_active, u.created_at as user_created_at, u.updated_at as user_updated_at, u.deleted_at as user_deleted_at,
                r.id as role_id, r.name, r.description, r.created_at as role_created_at, r.updated_at as role_updated_at, r.deleted_at as role_deleted_at
            FROM users u
            JOIN user_roles ur ON u.id = ur.user_id
            JOIN roles r ON ur.role_id = r.id
            WHERE u.id = $1
            "#,
            id
        )
        .fetch_all(&self.db_pool)
        .await?;

        if rows.is_empty() {
            return Err(sqlx::Error::RowNotFound)
        }

        let user_row = &rows[0];
        let user = User {
            id: user_row.user_id.clone(),
            email: user_row.email.clone(),
            password: user_row.password.clone(),
            email_verified_at: user_row.email_verified_at,
            last_login: user_row.last_login,
            is_active: user_row.is_active,
            created_at: user_row.user_created_at,
            updated_at: user_row.user_updated_at,
            deleted_at: user_row.user_deleted_at,
        };

        let roles = rows
            .into_iter()
            .map(|row| Role {
                id: row.role_id,
                name: row.name,
                description: row.description,
                created_at: row.role_created_at,
                updated_at: row.role_updated_at,
                deleted_at: row.role_deleted_at,
            })
            .collect();

        Ok(UserWithRoles { user: user, roles: roles })

    }

    pub async fn get_user_by_email_with_roles(&self, email: String) -> Result<UserWithRoles, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                u.id as user_id, u.email, u.password, u.email_verified_at, u.last_login, u.is_active, u.created_at as user_created_at, u.updated_at as user_updated_at, u.deleted_at as user_deleted_at,
                r.id as role_id, r.name, r.description, r.created_at as role_created_at, r.updated_at as role_updated_at, r.deleted_at as role_deleted_at
            FROM users u
            JOIN user_roles ur ON u.id = ur.user_id
            JOIN roles r ON ur.role_id = r.id
            WHERE u.email = $1
            "#,
            email
        )
        .fetch_all(&self.db_pool)
        .await?;

        if rows.is_empty() {
            return Err(sqlx::Error::RowNotFound)
        }

        let user_row = &rows[0];
        let user = User {
            id: user_row.user_id.clone(),
            email: user_row.email.clone(),
            password: user_row.password.clone(),
            email_verified_at: user_row.email_verified_at,
            last_login: user_row.last_login,
            is_active: user_row.is_active,
            created_at: user_row.user_created_at,
            updated_at: user_row.user_updated_at,
            deleted_at: user_row.user_deleted_at,
        };

        let roles = rows
            .into_iter()
            .map(|row| Role {
                id: row.role_id,
                name: row.name,
                description: row.description,
                created_at: row.role_created_at,
                updated_at: row.role_updated_at,
                deleted_at: row.role_deleted_at,
            })
            .collect();

        Ok(UserWithRoles { user: user, roles: roles })

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
