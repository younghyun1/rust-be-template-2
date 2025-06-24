use crate::domain::country_subdivision::dto::country_subdivision_dto::{
    CountrySubdivisionRequestDto, CountrySubdivisionResponseDto,
};
use crate::domain::country_subdivision::iso_country_subdivision::{
    IsoCountrySubdivision, IsoCountrySubdivisionInsert,
};

/// Conversion between IsoCountrySubdivision and CountrySubdivisionResponseDto
impl From<IsoCountrySubdivision> for CountrySubdivisionResponseDto {
    fn from(model: IsoCountrySubdivision) -> Self {
        CountrySubdivisionResponseDto {
            subdivision_id: model.subdivision_id,
            country_code: model.country_code,
            subdivision_code: model.subdivision_code,
            subdivision_name: model.subdivision_name,
            subdivision_type: model.subdivision_type,
        }
    }
}

/// Conversion from CountrySubdivisionRequestDto (API input) to IsoCountrySubdivisionInsert (VO)
impl From<CountrySubdivisionRequestDto> for IsoCountrySubdivisionInsert {
    fn from(dto: CountrySubdivisionRequestDto) -> Self {
        IsoCountrySubdivisionInsert {
            subdivision_id: dto.subdivision_id,
            country_code: dto.country_code,
            subdivision_code: dto.subdivision_code,
            subdivision_name: dto.subdivision_name,
            subdivision_type: dto.subdivision_type,
        }
    }
}

/// Optional: Conversion from IsoCountrySubdivisionInsert (VO) to CountrySubdivisionRequestDto
impl From<IsoCountrySubdivisionInsert> for CountrySubdivisionRequestDto {
    fn from(vo: IsoCountrySubdivisionInsert) -> Self {
        CountrySubdivisionRequestDto {
            subdivision_id: vo.subdivision_id,
            country_code: vo.country_code,
            subdivision_code: vo.subdivision_code,
            subdivision_name: vo.subdivision_name,
            subdivision_type: vo.subdivision_type,
        }
    }
}
