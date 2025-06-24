use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::impl_async_crud_repository_for;
use diesel_async::AsyncPgConnection;

/// A repository for `IsoCountry` that wraps a mutable reference to an active connection.
/// All methods require &mut self to allow mutable DB connection access.
/// This holds no pool and is meant to be constructed per operation/handler.
pub struct IsoCountryDieselAsyncPgRepository<'conn> {
    pub conn: &'conn mut AsyncPgConnection,
}

impl<'conn> IsoCountryDieselAsyncPgRepository<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

// Automatically implement the full AsyncRepository trait for IsoCountry
impl_async_crud_repository_for!(
    IsoCountryDieselAsyncPgRepository,
    IsoCountry,
    IsoCountryInsert,
    i32,
    crate::domain::schema::iso_country,
    country_code // <-- ADDED missing primary key column argument
);
