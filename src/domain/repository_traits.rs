// basic CRUD repository abstraction
pub trait Repository<ID, Entity, EntityInsert>: Send + Sync {
    fn create(&self, entity: EntityInsert) -> anyhow::Result<Entity>;
    fn create_many(&self, entities: Vec<EntityInsert>) -> anyhow::Result<Vec<Entity>>;
    fn read(&self, id: ID) -> anyhow::Result<Option<Entity>>;
    fn read_all(&self) -> anyhow::Result<Vec<Entity>>;
    fn update(&self, id: ID, entity: EntityInsert) -> anyhow::Result<Entity>;
    fn update_many<M>(&self, entities: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = (ID, EntityInsert)>;
    fn delete(&self, id: ID) -> anyhow::Result<Entity>;
    fn delete_many<M>(&self, ids: M) -> anyhow::Result<Vec<Entity>>
    where
        M: IntoIterator<Item = ID>;
}
