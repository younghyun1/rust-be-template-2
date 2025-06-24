use serde_derive::{Deserialize, Serialize};

/// Data Transfer Object for outputting Country Subdivision information (e.g., API response)
/// Data Transfer Object for outputting Country Subdivision information (e.g., API response)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountrySubdivisionResponseDto {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
}

/// Data Transfer Object for creating or updating Country Subdivision information (API input)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountrySubdivisionRequestDto {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
}

// DTO <-> Domain/VO conversion implementations
// are now in `country_subdivision/converter/country_subdivision_converter.rs`

// DTO <-> Domain/VO conversion implementations are now in `country_subdivision/converter/country_subdivision_converter.rs`
