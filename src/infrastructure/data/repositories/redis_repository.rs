use redis::AsyncCommands;
use serde_json::json;

use crate::domain::models::user_role::UserWithRoles;


#[derive(Clone)]
pub struct RedisRepository {
    redis_client: redis::aio::MultiplexedConnection,
}

impl RedisRepository {
    // RAM_ prefix is R.A.M -> Rust Auth Microservice :)
    pub fn new(redis_client: redis::aio::MultiplexedConnection) -> Self {
        Self {
            redis_client,
        }
    }

    pub async fn get_auth_token(&mut self, key: &String) -> Option<String> {
        let redis_key = format!("RAM_ACCESS_TOKEN_{}", key);

        self.redis_client.get(&redis_key).await.ok()
    }

    pub async fn set_auth_token(&mut self, key: String, token: &String) -> Result<(), redis::RedisError> {
        let redis_key = format!("RAM_ACCESS_TOKEN_{}", key);
        self.redis_client.set(&redis_key, token).await?;

        let in_week: i64 = 7 * 24 * 60 * 60;
        // set expiration
        self.redis_client.expire(&redis_key, in_week).await?;

        Ok(())
    }

    pub async fn set_auth_data(&mut self, key: &String, data: &UserWithRoles, access_token: &String) -> Result<(), redis::RedisError> {
        let token_key = format!("RAM_ACCESS_TOKEN_{}", key);
        let user_data_key = format!("RAM_USER_DATA_{}", key);
        self.redis_client.set(&token_key, access_token).await?;
        self.redis_client.set(&user_data_key, json!(data).to_string()).await?;

        let in_week: i64 = 7 * 24 * 60 * 60;
        let in_a_day: i64 = 24 * 60 * 60;
        // set expiration
        self.redis_client.expire(&token_key, in_week).await?;
        self.redis_client.expire(&user_data_key, in_a_day).await?;

        Ok(())
    }

    pub async fn remove_auth_data(&mut self, key: &String) -> Result<(), redis::RedisError> {
        let token_key = format!("RAM_ACCESS_TOKEN_{}", key);
        let user_data_key = format!("RAM_USER_DATA_{}", key);

        self.redis_client.del(&token_key).await?;
        self.redis_client.del(&user_data_key).await?;

        Ok(())
    }

    pub async fn get_user_data(&mut self, key: &String) -> Option<UserWithRoles> {
        let user_data_key = format!("RAM_USER_DATA_{}", key);

        let user_data: Option<String> = self
            .redis_client
            .get(&user_data_key)
            .await
            .ok();

        if let Some(user_data) = user_data {
            let user_data: UserWithRoles = serde_json::from_str(&user_data).ok()?;
            
            return Some(user_data)
        }

        None
    }

    pub async fn set_user_data(&mut self, key: String, data: &UserWithRoles) -> Result<(), redis::RedisError> {
        let user_data_key = format!("RAM_USER_DATA_{}", key);
        self.redis_client.set(&user_data_key, json!(data).to_string()).await?;

        let in_a_day: i64 = 24 * 60 * 60;
        // set expiration
        self.redis_client.expire(&user_data_key, in_a_day).await?;

        Ok(())
    }
}