use async_trait::async_trait;

/// A basic async CRUD repository abstraction where all methods require mutable access to self.
#[async_trait]
#[allow(unused)]
pub trait AsyncRepository<ID, Entity, EntityInsert>: Send + Sync {
    async fn create(&mut self, entity: EntityInsert) -> anyhow::Result<Entity>;
    async fn create_many(&mut self, entities: Vec<EntityInsert>) -> anyhow::Result<Vec<Entity>>;
    async fn read(&mut self, id: ID) -> anyhow::Result<Option<Entity>>;
    async fn read_all(&mut self) -> anyhow::Result<Vec<Entity>>;
    async fn update(&mut self, id: ID, entity: EntityInsert) -> anyhow::Result<Entity>;
    async fn update_many<M>(&mut self, entities: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = (ID, EntityInsert)> + Send + Sync;
    async fn delete(&mut self, id: ID) -> anyhow::Result<Entity>;
    async fn delete_many<M>(&mut self, ids: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = ID> + Send + Sync;
}

/// A synchronous CRUD repository abstraction.
#[allow(unused)]
pub trait SyncRepository<ID, Entity, EntityInsert>: Send + Sync {
    fn create(&mut self, entity: EntityInsert) -> anyhow::Result<Entity>;
    fn create_many(&mut self, entities: Vec<EntityInsert>) -> anyhow::Result<Vec<Entity>>;
    fn read(&mut self, id: ID) -> anyhow::Result<Option<Entity>>;
    fn read_all(&mut self) -> anyhow::Result<Vec<Entity>>;
    fn update(&mut self, id: ID, entity: EntityInsert) -> anyhow::Result<Entity>;
    fn update_many<M>(&mut self, entities: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = (ID, EntityInsert)> + Send + Sync;
    fn delete(&mut self, id: ID) -> anyhow::Result<Entity>;
    fn delete_many<M>(&mut self, ids: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = ID> + Send + Sync;
}
