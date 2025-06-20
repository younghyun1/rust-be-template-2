use serde_derive::{Deserialize, Serialize};

/// Data Transfer Object for outputting Country information publicly (e.g., API response)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryResponseDto {
    pub country_code: i32,
    pub country_alpha2: String,
    pub country_alpha3: String,
    pub country_eng_name: String,
    pub country_currency: i32,
    pub phone_prefix: String,
    pub country_flag: String,
    pub is_country: bool,
    pub country_primary_language: i32,
}

/// Data Transfer Object for creating or updating Country information (e.g., API input)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountryRequestDto {
    pub country_code: i32,
    pub country_alpha2: String,
    pub country_alpha3: String,
    pub country_eng_name: String,
    pub country_currency: i32,
    pub phone_prefix: String,
    pub country_flag: String,
    pub is_country: bool,
    pub country_primary_language: i32,
}

impl From<crate::domain::country::iso_country::IsoCountry> for CountryResponseDto {
    fn from(model: crate::domain::country::iso_country::IsoCountry) -> Self {
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

impl From<CountryRequestDto> for crate::domain::country::iso_country::IsoCountry {
    fn from(dto: CountryRequestDto) -> Self {
        crate::domain::country::iso_country::IsoCountry {
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
