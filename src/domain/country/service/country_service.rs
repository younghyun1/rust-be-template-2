use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::country::repository::iso_country_diesel_async_pg_repository::IsoCountryDieselAsyncPgRepository;
use crate::domain::country::repository::iso_country_static::IsoCountryStaticRepository;
use crate::domain::repository_traits::{AsyncRepository, SyncRepository};
use anyhow::Result;
use diesel_async::AsyncPgConnection;

/// CountryService is generic over repository implementation.
///
/// For database-backed operations, use `CountryService<IsoCountryDieselAsyncPgRepository<'conn>>`.
/// For static repo operations, use `CountryService<IsoCountryStaticRepository>`.
pub struct CountryService<R> {
    pub repo: R,
}

// ASYNC/DB CONSTRUCTORS & METHODS
impl<'conn> CountryService<IsoCountryDieselAsyncPgRepository<'conn>> {
    /// Construct a CountryService backed by an async Postgres Diesel repository.
    pub fn new_async_pg(conn: &'conn mut AsyncPgConnection) -> Self {
        let repo = IsoCountryDieselAsyncPgRepository::new(conn);
        Self { repo }
    }

    pub async fn get_country(&self, code: i32) -> Result<Option<IsoCountry>> {
        self.repo.read(code).await
    }

    pub async fn create_country(&self, country: IsoCountryInsert) -> Result<IsoCountry> {
        self.repo.create(country).await
    }

    pub async fn update_country(&self, code: i32, country: IsoCountryInsert) -> Result<IsoCountry> {
        self.repo.update(code, country).await
    }

    pub async fn delete_country(&self, code: i32) -> Result<IsoCountry> {
        self.repo.delete(code).await
    }

    pub async fn list_countries(&self) -> Result<Vec<IsoCountry>> {
        self.repo.read_all().await
    }
}

// STATIC REPO CONSTRUCTORS & METHODS
impl CountryService<IsoCountryStaticRepository> {
    /// Construct a CountryService backed by the static repository.
    pub fn new_static() -> Self {
        let repo = IsoCountryStaticRepository::new_static();
        Self { repo }
    }

    pub fn get_country(&self, code: i32) -> Result<Option<IsoCountry>> {
        self.repo.read(code)
    }

    pub fn create_country(&self, country: IsoCountryInsert) -> Result<IsoCountry> {
        self.repo.create(country)
    }

    pub fn update_country(&self, code: i32, country: IsoCountryInsert) -> Result<IsoCountry> {
        self.repo.update(code, country)
    }

    pub fn delete_country(&self, code: i32) -> Result<IsoCountry> {
        self.repo.delete(code)
    }

    pub fn list_countries(&self) -> Result<Vec<IsoCountry>> {
        self.repo.read_all()
    }
}

// USAGE:
//
// // Async DB example
// let mut conn = state.get_conn().await?;
// let service = CountryService::new_async_pg(&mut conn);
// let country = service.get_country(country_id).await?;
//
// // Static example
// let service = CountryService::new_static();
// let country = service.get_country(country_id)?;
