use crate::domain::country_subdivision::iso_country_subdivision::{
    IsoCountrySubdivision, IsoCountrySubdivisionInsert,
};
use crate::impl_async_crud_repository_for;
use diesel_async::AsyncPgConnection;

/// A repository for `IsoCountrySubdivision` that wraps a mutable reference to an active connection.
/// All methods require &mut self to allow mutable DB connection access.
/// This holds no pool and is meant to be constructed per operation/handler.
pub struct IsoCountrySubdivisionDieselAsyncPgRepository<'conn> {
    pub conn: &'conn mut AsyncPgConnection,
}

impl<'conn> IsoCountrySubdivisionDieselAsyncPgRepository<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
    // Implement custom methods for this domain here!
}

// Automatically implement the full AsyncRepository trait for IsoCountrySubdivision
impl_async_crud_repository_for!(
    IsoCountrySubdivisionDieselAsyncPgRepository,
    IsoCountrySubdivision,
    IsoCountrySubdivisionInsert,
    i32,
    crate::domain::schema::iso_country_subdivision,
    subdivision_id
);
