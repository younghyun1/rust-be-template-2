// basic async CRUD repository abstraction
use async_trait::async_trait;

#[async_trait]
pub trait AsyncRepository<ID, Entity, EntityInsert>: Send + Sync {
    async fn create(&self, entity: EntityInsert) -> anyhow::Result<Entity>;
    async fn create_many(&self, entities: Vec<EntityInsert>) -> anyhow::Result<Vec<Entity>>;
    async fn read(&self, id: ID) -> anyhow::Result<Option<Entity>>;
    async fn read_all(&self) -> anyhow::Result<Vec<Entity>>;
    async fn update(&self, id: ID, entity: EntityInsert) -> anyhow::Result<Entity>;
    async fn update_many<M>(&self, entities: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = (ID, EntityInsert)> + Send + Sync;
    async fn delete(&self, id: ID) -> anyhow::Result<Entity>;
    async fn delete_many<M>(&self, ids: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = ID> + Send + Sync;
}
