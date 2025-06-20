use diesel_async::AsyncConnection;

use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::country::repository::iso_country_repository::IsoCountryRepository;
use crate::domain::repository_traits::AsyncRepository;
use anyhow::Result;
use async_trait::async_trait;

pub struct IsoCountryDieselAsyncRepository<Conn>
where
    Conn: AsyncConnection + Send + Sync + 'static,
{
    pub conn: Conn,
}

// Implement the required async CRUD trait for compiler satisfaction
#[async_trait]
impl<Conn> AsyncRepository<i32, IsoCountry, IsoCountryInsert>
    for IsoCountryDieselAsyncRepository<Conn>
where
    Conn: AsyncConnection + Send + Sync + 'static,
{
    async fn create(&self, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        todo!()
    }
    async fn create_many(&self, _entities: Vec<IsoCountryInsert>) -> Result<Vec<IsoCountry>> {
        todo!()
    }
    async fn read(&self, _id: i32) -> Result<Option<IsoCountry>> {
        todo!()
    }
    async fn read_all(&self) -> Result<Vec<IsoCountry>> {
        todo!()
    }
    async fn update(&self, _id: i32, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        todo!()
    }
    async fn update_many<M>(&self, _entities: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = (i32, IsoCountryInsert)> + Send + Sync,
    {
        todo!()
    }
    async fn delete(&self, _id: i32) -> Result<IsoCountry> {
        todo!()
    }
    async fn delete_many<M>(&self, _ids: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = i32> + Send + Sync,
    {
        todo!()
    }
}

impl<Conn> IsoCountryRepository<Conn> for IsoCountryDieselAsyncRepository<Conn>
where
    Conn: AsyncConnection + Send + Sync + 'static,
{
    fn with_conn(conn: Conn) -> Self
    where
        Self: Sized,
    {
        IsoCountryDieselAsyncRepository { conn }
    }
}

impl<Conn> IsoCountryDieselAsyncRepository<Conn>
where
    Conn: AsyncConnection + Send + Sync + 'static,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}
