use async_trait::async_trait;

/// A macro to implement the `AsyncRepository` trait for a given Diesel entity.
///
/// This reduces boilerplate for standard CRUD operations.
///
/// # Arguments
///
/// * `$repo_struct`: The name of the repository struct to implement the trait for.
/// * `$entity`: The type of the queryable entity struct (e.g., `IsoCountry`).
/// * `$insertable`: The type of the insertable/updatable struct (e.g., `IsoCountryInsert`).
/// * `$id_type`: The Rust type of the primary key (e.g., `i32`).
/// * `$schema_module`: The corresponding module from `schema.rs` (e.g., `crate::domain::schema::iso_country`).
/// * `$pk_column`: The identifier of the primary key column in the schema's dsl (e.g., `country_code`).
#[macro_export]
macro_rules! impl_async_crud_repository_for {
    (
        $repo_struct:ident,
        $entity:ty,
        $insertable:ty,
        $id_type:ty,
        $schema_module:path,
        $pk_column:ident
    ) => {
        // This const block creates a new scope, allowing us to import the schema module
        // under a consistent alias (`schema`) to avoid macro parsing issues with `::`.
        const _: () = {
            use $schema_module as schema; // Alias the provided schema path.

            #[async_trait::async_trait]
            impl<'conn>
                $crate::domain::repository_traits::AsyncRepository<$id_type, $entity, $insertable>
                for $repo_struct<'conn>
            {
                async fn create(&mut self, entity: $insertable) -> anyhow::Result<$entity> {
                    use anyhow::anyhow;
                    use diesel_async::RunQueryDsl;

                    diesel::insert_into(schema::table)
                        .values(entity)
                        .returning(schema::all_columns)
                        .get_result(self.conn)
                        .await
                        .map_err(|e| anyhow!("DB creation error: {}", e))
                }

                async fn create_many(
                    &mut self,
                    entities: Vec<$insertable>,
                ) -> anyhow::Result<Vec<$entity>> {
                    use anyhow::anyhow;
                    use diesel_async::RunQueryDsl;

                    if entities.is_empty() {
                        return Ok(vec![]);
                    }
                    diesel::insert_into(schema::table)
                        .values(entities)
                        .returning(schema::all_columns)
                        .get_results(self.conn)
                        .await
                        .map_err(|e| anyhow!("DB batch creation error: {}", e))
                }

                async fn read(&mut self, id: $id_type) -> anyhow::Result<Option<$entity>> {
                    use anyhow::anyhow;
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;

                    schema::table
                        .find(id)
                        .first(self.conn)
                        .await
                        .optional()
                        .map_err(|e| anyhow!("DB read error: {}", e))
                }

                async fn read_all(&mut self) -> anyhow::Result<Vec<$entity>> {
                    use anyhow::anyhow;
                    use diesel_async::RunQueryDsl;

                    schema::table
                        .load(self.conn)
                        .await
                        .map_err(|e| anyhow!("DB read_all error: {}", e))
                }

                async fn update(
                    &mut self,
                    id: $id_type,
                    entity: $insertable,
                ) -> anyhow::Result<$entity> {
                    use anyhow::anyhow;
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;

                    diesel::update(schema::table.find(id))
                        .set(entity)
                        .returning(schema::all_columns)
                        .get_result(self.conn)
                        .await
                        .map_err(|e| anyhow!("DB update error: {}", e))
                }

                async fn update_many<M>(&mut self, _entities: M) -> anyhow::Result<Vec<$entity>>
                where
                    M: IntoIterator<Item = ($id_type, $insertable)> + Send + Sync,
                {
                    use anyhow::anyhow;
                    Err(anyhow!(
                        "Generic `update_many` is not implemented due to its complexity."
                    ))
                }

                async fn delete(&mut self, id: $id_type) -> anyhow::Result<$entity> {
                    use anyhow::anyhow;
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;

                    diesel::delete(schema::table.find(id))
                        .returning(schema::all_columns)
                        .get_result(self.conn)
                        .await
                        .map_err(|e| anyhow!("DB delete error: {}", e))
                }

                async fn delete_many<M>(&mut self, ids: M) -> anyhow::Result<Vec<$entity>>
                where
                    M: IntoIterator<Item = $id_type> + Send + Sync,
                {
                    use anyhow::anyhow;
                    use diesel::prelude::*;
                    use diesel_async::RunQueryDsl;

                    let pks: Vec<$id_type> = ids.into_iter().collect();
                    if pks.is_empty() {
                        return Ok(vec![]);
                    }
                    diesel::delete(schema::table.filter(schema::dsl::$pk_column.eq_any(pks)))
                        .returning(schema::all_columns)
                        .get_results(self.conn)
                        .await
                        .map_err(|e| anyhow!("DB batch delete error: {}", e))
                }
            }
        }; // End of const block
    };
}

/// A basic async CRUD repository abstraction where all methods require mutable access to self.
#[async_trait]
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
#[async_trait]
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
