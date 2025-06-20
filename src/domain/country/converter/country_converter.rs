use crate::domain::country::dto::country_dto::{CountryRequestDto, CountryResponseDto};
use crate::domain::country::iso_country::{IsoCountry, IsoCountryInsert};

/// Conversion between IsoCountry and CountryResponseDto
impl From<IsoCountry> for CountryResponseDto {
    fn from(model: IsoCountry) -> Self {
        CountryResponseDto {
            country_code: model.country_code,
            country_alpha2: model.country_alpha2,
            country_alpha3: model.country_alpha3,
            country_eng_name: model.country_eng_name,
            country_currency: model.country_currency,
            phone_prefix: model.phone_prefix,
            country_flag: model.country_flag,
            is_country: model.is_country,
            country_primary_language: model.country_primary_language,
        }
    }
}

/// Conversion from CountryRequestDto (API input) to IsoCountryInsert (VO)
impl From<CountryRequestDto> for IsoCountryInsert {
    fn from(dto: CountryRequestDto) -> Self {
        IsoCountryInsert {
            country_code: dto.country_code,
            country_alpha2: dto.country_alpha2,
            country_alpha3: dto.country_alpha3,
            country_eng_name: dto.country_eng_name,
            country_currency: dto.country_currency,
            phone_prefix: dto.phone_prefix,
            country_flag: dto.country_flag,
            is_country: dto.is_country,
            country_primary_language: dto.country_primary_language,
        }
    }
}

/// Optional: Conversion from IsoCountryInsert (VO) to CountryRequestDto
impl From<IsoCountryInsert> for CountryRequestDto {
    fn from(vo: IsoCountryInsert) -> Self {
        CountryRequestDto {
            country_code: vo.country_code,
            country_alpha2: vo.country_alpha2,
            country_alpha3: vo.country_alpha3,
            country_eng_name: vo.country_eng_name,
            country_currency: vo.country_currency,
            phone_prefix: vo.phone_prefix,
            country_flag: vo.country_flag,
            is_country: vo.is_country,
            country_primary_language: vo.country_primary_language,
        }
    }
}
