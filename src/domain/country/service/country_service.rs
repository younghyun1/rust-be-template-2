use crate::domain::country::iso_country::IsoCountry;
use crate::domain::country::repository::iso_country_diesel_async_repository::IsoCountryDieselAsyncRepository;
use anyhow::Result;
use async_trait::async_trait;
use diesel_async::AsyncConnection;

/// Dyn-compatible async trait using async-trait macro for repository pattern.
#[async_trait]
pub trait IsoCountryRepositoryDyn: Send + Sync {
    async fn read(&self, country_code: i32) -> Result<Option<IsoCountry>>;
    // You can add more async methods here as needed (create, update, delete, etc.)
}

/// Blanket impl of the trait object for your repository struct (generic over connection).
#[async_trait]
impl<C> IsoCountryRepositoryDyn for IsoCountryDieselAsyncRepository<C>
where
    C: AsyncConnection + Send + Sync + 'static,
{
    async fn read(&self, country_code: i32) -> Result<Option<IsoCountry>> {
        // Real implementation should query your database.
        // This is a stub for demonstration.
        Ok(None)
    }
}

/// Example service using the dyn repo (Boxed)
pub struct CountryService {
    repo: Box<dyn IsoCountryRepositoryDyn>,
}

impl CountryService {
    pub fn new(repo: Box<dyn IsoCountryRepositoryDyn>) -> Self {
        Self { repo }
    }

    pub async fn get_country(&self, code: i32) -> Result<Option<IsoCountry>> {
        self.repo.read(code).await
    }
}

// Usage example (not actual test, just to show the idea):
// let repo = Box::new(IsoCountryDieselAsyncRepository::new(conn)) as Box<dyn IsoCountryRepositoryDyn>;
// let service = CountryService::new(repo);
// let country = service.get_country(1).await?;
