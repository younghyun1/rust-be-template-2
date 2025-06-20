use diesel_async::AsyncConnection;

use crate::domain::country::repository::iso_country_repository::IsoCountryRepository;

pub struct IsoCountryDieselAsyncRepository<Conn>
where
    Conn: AsyncConnection + Send + Sync + 'static,
{
    pub conn: Conn,
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
    Conn: AsyncConnection + Send + 'static,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }
}
