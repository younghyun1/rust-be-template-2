use crate::domain::country_subdivision::iso_country_subdivision::{IsoCountrySubdivision, IsoCountrySubdivisionInsert};
use crate::domain::country_subdivision::repository::iso_country_subdivision_diesel_async_pg_repository::IsoCountrySubdivisionDieselAsyncPgRepository;
use crate::domain::country_subdivision::repository::iso_country_subdivision_static::IsoCountrySubdivisionStaticRepository;
use crate::domain::repository_traits::{AsyncRepository, SyncRepository};
use anyhow::Result;
use diesel_async::AsyncPgConnection;

/// CountrySubdivisionService is generic over repository implementation.
///
/// For database-backed operations, use `CountrySubdivisionService<IsoCountrySubdivisionDieselAsyncPgRepository<'conn>>`.
/// For static repo operations, use `CountrySubdivisionService<IsoCountrySubdivisionStaticRepository>`.
pub struct CountrySubdivisionService<R> {
    pub repo: R,
}

// ASYNC/DB CONSTRUCTORS & METHODS
impl<'conn> CountrySubdivisionService<IsoCountrySubdivisionDieselAsyncPgRepository<'conn>> {
    /// Construct a CountrySubdivisionService backed by an async Postgres Diesel repository.
    pub fn new_async_pg(conn: &'conn mut AsyncPgConnection) -> Self {
        let repo = IsoCountrySubdivisionDieselAsyncPgRepository::new(conn);
        Self { repo }
    }

    pub async fn get_subdivision(
        &mut self,
        subdivision_id: i32,
    ) -> Result<Option<IsoCountrySubdivision>> {
        self.repo.read(subdivision_id).await
    }

    pub async fn create_subdivision(
        &mut self,
        subdivision: IsoCountrySubdivisionInsert,
    ) -> Result<IsoCountrySubdivision> {
        self.repo.create(subdivision).await
    }

    pub async fn update_subdivision(
        &mut self,
        subdivision_id: i32,
        subdivision: IsoCountrySubdivisionInsert,
    ) -> Result<IsoCountrySubdivision> {
        self.repo.update(subdivision_id, subdivision).await
    }

    pub async fn delete_subdivision(
        &mut self,
        subdivision_id: i32,
    ) -> Result<IsoCountrySubdivision> {
        self.repo.delete(subdivision_id).await
    }

    pub async fn list_subdivisions(&mut self) -> Result<Vec<IsoCountrySubdivision>> {
        self.repo.read_all().await
    }
}

// STATIC REPO CONSTRUCTORS & METHODS
impl CountrySubdivisionService<IsoCountrySubdivisionStaticRepository> {
    /// Construct a CountrySubdivisionService backed by the static repository.
    pub fn new_static() -> Self {
        let repo = IsoCountrySubdivisionStaticRepository::new_static();
        Self { repo }
    }

    pub fn get_subdivision(
        &mut self,
        subdivision_id: i32,
    ) -> Result<Option<IsoCountrySubdivision>> {
        self.repo.read(subdivision_id)
    }

    pub fn create_subdivision(
        &mut self,
        subdivision: IsoCountrySubdivisionInsert,
    ) -> Result<IsoCountrySubdivision> {
        self.repo.create(subdivision)
    }

    pub fn update_subdivision(
        &mut self,
        subdivision_id: i32,
        subdivision: IsoCountrySubdivisionInsert,
    ) -> Result<IsoCountrySubdivision> {
        self.repo.update(subdivision_id, subdivision)
    }

    pub fn delete_subdivision(&mut self, subdivision_id: i32) -> Result<IsoCountrySubdivision> {
        self.repo.delete(subdivision_id)
    }

    pub fn list_subdivisions(&mut self) -> Result<Vec<IsoCountrySubdivision>> {
        self.repo.read_all()
    }
}

// USAGE:
//
// // Async DB example
// let mut conn = state.get_conn().await?;
// let service = CountrySubdivisionService::new_async_pg(&mut conn);
// let subdivision = service.get_subdivision(subdivision_id).await?;
//
// // Static example
// let service = CountrySubdivisionService::new_static();
// let subdivision = service.get_subdivision(subdivision_id)?;
