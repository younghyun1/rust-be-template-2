use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::repository_traits::AsyncRepository;
use anyhow::Result;
use async_trait::async_trait;
use diesel_async::AsyncPgConnection;

/// A repository that wraps a mutable reference to an active connection.
/// This holds no pool and is meant to be constructed per operation/handler.
pub struct IsoCountryDieselAsyncPgRepository<'conn> {
    pub conn: &'conn mut AsyncPgConnection,
}

impl<'conn> IsoCountryDieselAsyncPgRepository<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'conn> AsyncRepository<i32, IsoCountry, IsoCountryInsert>
    for IsoCountryDieselAsyncPgRepository<'conn>
{
    async fn create(&self, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        // Example: use self.conn for Diesel queries here
        // e.g. diesel::insert_into(...).execute(self.conn).await?;
        todo!()
    }

    async fn create_many(&self, _entities: Vec<IsoCountryInsert>) -> Result<Vec<IsoCountry>> {
        // Use self.conn
        todo!()
    }

    async fn read(&self, _id: i32) -> Result<Option<IsoCountry>> {
        // Use self.conn
        todo!()
    }

    async fn read_all(&self) -> Result<Vec<IsoCountry>> {
        // Use self.conn
        todo!()
    }

    async fn update(&self, _id: i32, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        // Use self.conn
        todo!()
    }

    async fn update_many<M>(&self, _entities: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = (i32, IsoCountryInsert)> + Send + Sync,
    {
        // Use self.conn
        todo!()
    }

    async fn delete(&self, _id: i32) -> Result<IsoCountry> {
        // Use self.conn
        todo!()
    }

    async fn delete_many<M>(&self, _ids: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = i32> + Send + Sync,
    {
        // Use self.conn
        todo!()
    }
}
