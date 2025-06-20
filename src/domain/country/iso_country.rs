use diesel::{Queryable, prelude::QueryableByName};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, QueryableByName, Queryable)]
#[diesel(table_name = iso_country)]
#[diesel(primary_key(country_code))]
pub struct IsoCountry {
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = country_code)]
    pub country_code: i32, // ISO-defined, not autoincremented
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = country_alpha2)]
    pub country_alpha2: String,
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = country_alpha3)]
    pub country_alpha3: String,
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = country_eng_name)]
    pub country_eng_name: String,
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = country_currency)]
    pub country_currency: i32,
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = phone_prefix)]
    pub phone_prefix: String,
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = country_flag)]
    pub country_flag: String,
    #[diesel(sql_type = diesel::sql_types::Bool, column_name = is_country)]
    pub is_country: bool,
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = country_primary_language)]
    pub country_primary_language: i32,
}

pub struct IsoCountryInsert {
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
