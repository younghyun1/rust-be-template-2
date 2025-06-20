/// Static repository interface for IsoCountry records at build time.
///
/// Re-exports generated static and lookup functions.
/// Do not edit the generated file directly; see build.rs.
pub mod generated {
    include!("iso_country_static_gen.rs");
}

pub use generated::{ISO_COUNTRIES, by_alpha2, by_alpha3, by_code};

use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert, IsoCountryStatic};
use crate::domain::repository_traits::SyncRepository;
use anyhow::Result;

/// Static-specific lookup extension for IsoCountry
pub trait IsoCountryStaticLookups {
    fn by_code(&self, code: i32) -> Option<&'static IsoCountryStatic>;
    fn by_alpha2(&self, alpha2: &str) -> Option<&'static IsoCountryStatic>;
    fn by_alpha3(&self, alpha3: &str) -> Option<&'static IsoCountryStatic>;
}

/// Static implementation of repository trait for &'static [IsoCountry]
pub struct IsoCountryStaticRepository;

impl IsoCountryStaticRepository {
    pub fn new_static() -> Self {
        IsoCountryStaticRepository
    }
}

impl IsoCountryStaticLookups for IsoCountryStaticRepository {
    fn by_code(&self, code: i32) -> Option<&'static IsoCountryStatic> {
        by_code(code)
    }
    fn by_alpha2(&self, alpha2: &str) -> Option<&'static IsoCountryStatic> {
        by_alpha2(alpha2)
    }
    fn by_alpha3(&self, alpha3: &str) -> Option<&'static IsoCountryStatic> {
        by_alpha3(alpha3)
    }
}

impl IsoCountryStatic {
    pub fn to_owned(&self) -> IsoCountry {
        IsoCountry {
            country_code: self.country_code,
            country_alpha2: self.country_alpha2.to_string(),
            country_alpha3: self.country_alpha3.to_string(),
            country_eng_name: self.country_eng_name.to_string(),
            country_currency: self.country_currency,
            phone_prefix: self.phone_prefix.to_string(),
            country_flag: self.country_flag.to_string(),
            is_country: self.is_country,
            country_primary_language: self.country_primary_language,
        }
    }
}

impl SyncRepository<i32, IsoCountry, IsoCountryInsert> for IsoCountryStaticRepository {
    fn create(&mut self, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        Err(anyhow::anyhow!("Cannot create into static repository"))
    }

    fn create_many(&mut self, _entities: Vec<IsoCountryInsert>) -> Result<Vec<IsoCountry>> {
        Err(anyhow::anyhow!("Cannot create_many into static repository"))
    }

    fn read(&mut self, id: i32) -> Result<Option<IsoCountry>> {
        Ok(by_code(id).map(|s| s.to_owned()))
    }

    fn read_all(&mut self) -> Result<Vec<IsoCountry>> {
        Ok(ISO_COUNTRIES.iter().map(|s| s.to_owned()).collect())
    }

    fn update(&mut self, _id: i32, _entity: IsoCountryInsert) -> Result<IsoCountry> {
        Err(anyhow::anyhow!("Cannot update static repository"))
    }

    fn update_many<M>(&mut self, _entities: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = (i32, IsoCountryInsert)> + Send + Sync,
    {
        Err(anyhow::anyhow!("Cannot update_many static repository"))
    }

    fn delete(&mut self, _id: i32) -> Result<IsoCountry> {
        Err(anyhow::anyhow!("Cannot delete from static repository"))
    }

    fn delete_many<M>(&mut self, _ids: M) -> Result<Vec<IsoCountry>>
    where
        M: IntoIterator<Item = i32> + Send + Sync,
    {
        Err(anyhow::anyhow!("Cannot delete_many from static repository"))
    }
}
