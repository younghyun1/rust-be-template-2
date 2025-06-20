use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::repository_traits::AsyncRepository;

pub trait IsoCountryRepository<Conn>: AsyncRepository<i32, IsoCountry, IsoCountryInsert> {
    fn with_conn(conn: Conn) -> Self
    where
        Self: Sized;
}
