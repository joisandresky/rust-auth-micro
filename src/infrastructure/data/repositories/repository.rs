
#[async_trait::async_trait]
pub trait Repository<E, ID> {
    async fn get_all(&self) -> Result<Vec<E>, sqlx::Error>;
    async fn get_by_id(&self, id: ID) -> Result<E, sqlx::Error>;
    async fn create(&self, entity: E) -> Result<E, sqlx::Error>;
    async fn update(&self, entity: E) -> Result<E, sqlx::Error>;
    async fn delete_by_id(&self, id: ID) -> Result<(), sqlx::Error>;
}