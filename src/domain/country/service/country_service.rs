use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};
use crate::domain::country::repository::iso_country_diesel_async_pg_repository::IsoCountryDieselAsyncPgRepository;
use crate::domain::repository_traits::AsyncRepository;
use anyhow::Result;
use diesel_async::AsyncPgConnection;

/// CountryService using a repository that borrows a mutable connection reference for each operation.
/// Construct a new instance per request, pass the &mut connection from your handler.
/// Example usage pattern:
///
/// ```rust,ignore
/// let mut conn = state.get_conn().await?;
/// let service = CountryService::new(&mut conn);
/// let country = service.get_country(country_id).await?;
/// ```
pub struct CountryService<'conn> {
    pub repo: IsoCountryDieselAsyncPgRepository<'conn>,
}

impl<'conn> CountryService<'conn> {
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

// Usage in Axum handler or elsewhere:
// let mut conn = state.get_conn().await?;
// let service = CountryService::new(&mut conn);
// let country = service.get_country(country_id).await?;
