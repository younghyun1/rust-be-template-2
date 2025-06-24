use crate::domain::country_subdivision::iso_country_subdivision::{IsoCountrySubdivision, IsoCountrySubdivisionInsert};
use crate::domain::repository_traits::AsyncRepository;
use diesel_async::AsyncPgConnection;

/// Trait for accessing ISO country subdivision database records asynchronously with Diesel.
///
/// This trait is intended for repositories that operate over a mutable reference to an
/// `AsyncPgConnection`. It is meant for static dispatch, created per operation.
pub trait IsoCountrySubdivisionAsyncRepository<'conn>:
    AsyncRepository<i32, IsoCountrySubdivision, IsoCountrySubdivisionInsert>
{
    /// Construct a repository with a mutable reference to a connection.
    fn new(conn: &'conn mut AsyncPgConnection) -> Self
    where
        Self: Sized;
}
