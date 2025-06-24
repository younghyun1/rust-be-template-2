/// Static repository interface for IsoCountrySubdivision records at build time.
///
/// Re-exports generated static and lookup functions.
/// Do not edit the generated file directly; see build.rs.
pub mod generated {
    include!("iso_country_subdivision_static_gen.rs");
}

pub use generated::{ISO_COUNTRY_SUBDIVISIONS, by_country_code, by_id};

use crate::domain::country_subdivision::iso_country_subdivision::{
    IsoCountrySubdivision, IsoCountrySubdivisionInsert, IsoCountrySubdivisionStatic,
};
use crate::domain::repository_traits::SyncRepository;
use anyhow::Result;

/// Static-specific lookup extension for IsoCountrySubdivision
pub trait IsoCountrySubdivisionStaticLookups {
    fn by_id(&self, subdivision_id: i32) -> Option<&'static IsoCountrySubdivisionStatic>;
    fn by_country_code(&self, country_code: i32)
    -> &'static [&'static IsoCountrySubdivisionStatic];
}

/// Static implementation of repository trait for &'static [IsoCountrySubdivision]
pub struct IsoCountrySubdivisionStaticRepository;

impl IsoCountrySubdivisionStaticRepository {
    pub fn new_static() -> Self {
        IsoCountrySubdivisionStaticRepository
    }
}

impl IsoCountrySubdivisionStaticLookups for IsoCountrySubdivisionStaticRepository {
    fn by_id(&self, subdivision_id: i32) -> Option<&'static IsoCountrySubdivisionStatic> {
        by_id(subdivision_id)
    }
    fn by_country_code(
        &self,
        country_code: i32,
    ) -> &'static [&'static IsoCountrySubdivisionStatic] {
        by_country_code(country_code)
    }
}

impl IsoCountrySubdivisionStatic {
    pub fn to_owned(&self) -> IsoCountrySubdivision {
        IsoCountrySubdivision {
            subdivision_id: self.subdivision_id,
            country_code: self.country_code,
            subdivision_code: self.subdivision_code.to_string(),
            subdivision_name: self.subdivision_name.to_string(),
            subdivision_type: self.subdivision_type.map(|s| s.to_string()),
        }
    }
}

impl SyncRepository<i32, IsoCountrySubdivision, IsoCountrySubdivisionInsert>
    for IsoCountrySubdivisionStaticRepository
{
    fn create(&mut self, _entity: IsoCountrySubdivisionInsert) -> Result<IsoCountrySubdivision> {
        Err(anyhow::anyhow!("Cannot create into static repository"))
    }

    fn create_many(
        &mut self,
        _entities: Vec<IsoCountrySubdivisionInsert>,
    ) -> Result<Vec<IsoCountrySubdivision>> {
        Err(anyhow::anyhow!("Cannot create_many into static repository"))
    }

    fn read(&mut self, id: i32) -> Result<Option<IsoCountrySubdivision>> {
        Ok(by_id(id).map(|s| s.to_owned()))
    }

    fn read_all(&mut self) -> Result<Vec<IsoCountrySubdivision>> {
        Ok(ISO_COUNTRY_SUBDIVISIONS
            .iter()
            .map(|s| s.to_owned())
            .collect())
    }

    fn update(
        &mut self,
        _id: i32,
        _entity: IsoCountrySubdivisionInsert,
    ) -> Result<IsoCountrySubdivision> {
        Err(anyhow::anyhow!("Cannot update static repository"))
    }

    fn update_many<M>(&mut self, _entities: M) -> Result<Vec<IsoCountrySubdivision>>
    where
        M: IntoIterator<Item = (i32, IsoCountrySubdivisionInsert)> + Send + Sync,
    {
        Err(anyhow::anyhow!("Cannot update_many static repository"))
    }

    fn delete(&mut self, _id: i32) -> Result<IsoCountrySubdivision> {
        Err(anyhow::anyhow!("Cannot delete from static repository"))
    }

    fn delete_many<M>(&mut self, _ids: M) -> Result<Vec<IsoCountrySubdivision>>
    where
        M: IntoIterator<Item = i32> + Send + Sync,
    {
        Err(anyhow::anyhow!("Cannot delete_many from static repository"))
    }
}
