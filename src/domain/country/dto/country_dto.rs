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

// DTO <-> Domain/VO conversion implementations are now in `country/converter/country_converter.rs`
