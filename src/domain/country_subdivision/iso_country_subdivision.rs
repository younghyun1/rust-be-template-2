use diesel::{
    Insertable, Queryable,
    prelude::{AsChangeset, QueryableByName},
};
use serde_derive::{Deserialize, Serialize};

use crate::domain::schema::iso_country_subdivision;

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName, Queryable)]
#[diesel(table_name = iso_country_subdivision)]
#[diesel(primary_key(subdivision_id))]
pub struct IsoCountrySubdivision {
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = subdivision_id)]
    pub subdivision_id: i32,
    #[diesel(sql_type = diesel::sql_types::Integer, column_name = country_code)]
    pub country_code: i32,
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = subdivision_code)]
    pub subdivision_code: String,
    #[diesel(sql_type = diesel::sql_types::VarChar, column_name = subdivision_name)]
    pub subdivision_name: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::VarChar>, column_name = subdivision_type)]
    pub subdivision_type: Option<String>,
}

/// For static bundling and lookup (all &'static).
pub struct IsoCountrySubdivisionStatic {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: &'static str,
    pub subdivision_name: &'static str,
    pub subdivision_type: Option<&'static str>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = iso_country_subdivision)]
pub struct IsoCountrySubdivisionInsert {
    pub subdivision_id: i32,
    pub country_code: i32,
    pub subdivision_code: String,
    pub subdivision_name: String,
    pub subdivision_type: Option<String>,
}
