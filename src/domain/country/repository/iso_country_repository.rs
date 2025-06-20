use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::repository_traits::Repository;

pub trait IsoCountryRepository<Conn>: Repository<i32, IsoCountry, IsoCountryInsert> {
    fn with_conn(conn: Conn) -> Self
    where
        Self: Sized;
}
