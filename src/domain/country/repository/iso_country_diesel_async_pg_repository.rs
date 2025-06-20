use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::repository_traits::AsyncRepository;
use crate::domain::schema::iso_country;

use anyhow::Result;
use async_trait::async_trait;
use diesel::dsl::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

/// A repository that wraps a mutable reference to an active connection.
/// All methods require &mut self to allow mutable DB connection access.
/// This holds no pool and is meant to be constructed per operation/handler.
pub struct IsoCountryDieselAsyncPgRepository<'conn> {
    pub conn: &'conn mut AsyncPgConnection,
}

// Custom-repository equivalent
impl<'conn> IsoCountryDieselAsyncPgRepository<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'conn> AsyncRepository<i32, IsoCountry, IsoCountryInsert>
    for IsoCountryDieselAsyncPgRepository<'conn>
{
    /// Example working method using &mut self for Diesel async.
    async fn create(&mut self, entity: IsoCountryInsert) -> Result<IsoCountry> {
        let res = diesel::insert_into(iso_country::table)
            .values(&entity)
            .returning(iso_country::all_columns)
            .get_result::<IsoCountry>(self.conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => anyhow::anyhow!("Country code or alpha2/alpha3 not unique: {}", e),
                _ => anyhow::anyhow!("DB insertion error: {}", e),
            })?;
        Ok(res)
    }

    async fn create_many(&mut self, entities: Vec<IsoCountryInsert>) -> Result<Vec<IsoCountry>> {
        if entities.is_empty() {
            return Ok(vec![]);
        }

        let res = insert_into(iso_country::table)
            .values(&entities)
            .returning(iso_country::all_columns)
            .get_results::<IsoCountry>(self.conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => anyhow::anyhow!(
                    "At least one country code or alpha2/alpha3 not unique: {}",
                    e
                ),
                _ => anyhow::anyhow!("DB batch insertion error: {}", e),
            })?;
        Ok(res)
    }

    async fn read(&mut self, _id: i32) -> Result<Option<IsoCountry>> {
        // Use self.conn
        todo!()
    }

    async fn read_all(&mut self) -> Result<Vec<IsoCountry>> {
        // Use self.conn
        todo!()
    }

    async fn update(&mut self, _id: i32, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        // Use self.conn
        todo!()
    }

    async fn update_many<M>(&mut self, _entities: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = (i32, IsoCountryInsert)> + Send + Sync,
    {
        // Use self.conn
        todo!()
    }

    async fn delete(&mut self, _id: i32) -> Result<IsoCountry> {
        // Use self.conn
        todo!()
    }

    async fn delete_many<M>(&mut self, _ids: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = i32> + Send + Sync,
    {
        // Use self.conn
        todo!()
    }
}
