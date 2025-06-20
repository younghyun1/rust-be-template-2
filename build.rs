use std::{env, fs, path::Path};

use postgres::{Client, NoTls};

fn main() {
    dotenvy::dotenv().ok();

    let out_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");

    // Get DB info from .env variables for connection
    let db_host = env::var("DB_HOST").expect("DB_HOST not found");
    let db_port = env::var("DB_PORT").expect("DB_PORT not found");
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME not found");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found");
    let db_name = env::var("DB_NAME").expect("DB_NAME not found");

    let db_url = if db_host.starts_with('/') {
        format!(
            "postgresql://{}:{}@{}/{}?host={}&port={}",
            db_username, db_password, "localhost", db_name, db_host, db_port
        )
    } else {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            db_username, db_password, db_host, db_port, db_name
        )
    };
    let mut client = Client::connect(&db_url, NoTls).expect("Could not connect to Postgres");

    // Query all rows from iso_country and emit them as Rust code for a const [IsoCountry; N]
    let static_out_path = Path::new(&out_dir)
        .join("src/domain/country/repository/iso_country_static_gen.rs");

    let rows = client
        .query(
            "SELECT country_code, country_alpha2, country_alpha3, country_eng_name, country_currency, \
                    phone_prefix, country_flag, is_country, country_primary_language \
             FROM iso_country ORDER BY country_code",
            &[],
        )
        .expect("Failed to query iso_country");

    // Serialize each DB row as IsoCountryStatic for Rust code
    let mut countries = String::new();
    for row in &rows {
        let country_code: i32 = row.get(0);
        let country_alpha2: String = row.get(1);
        let country_alpha3: String = row.get(2);
        let country_eng_name: String = row.get(3);
        let country_currency: i32 = row.get(4);
        let phone_prefix: String = row.get(5);
        let country_flag: String = row.get(6);
        let is_country: bool = row.get(7);
        let country_primary_language: i32 = row.get(8);
        let s = format!(
            "    IsoCountryStatic {{
        country_code: {country_code},
        country_alpha2: {alpha2},
        country_alpha3: {alpha3},
        country_eng_name: {eng_name},
        country_currency: {country_currency},
        phone_prefix: {phone_prefix},
        country_flag: {country_flag},
        is_country: {is_country},
        country_primary_language: {country_primary_language},
    }},\n",
            country_code = country_code,
            alpha2 = literal_str(&country_alpha2),
            alpha3 = literal_str(&country_alpha3),
            eng_name = literal_str(&country_eng_name),
            country_currency = country_currency,
            phone_prefix = literal_str(&phone_prefix),
            country_flag = literal_str(&country_flag),
            is_country = is_country,
            country_primary_language = country_primary_language
        );
        countries.push_str(&s);
    }

    // Formats a Rust 'static str literal (with proper escaping).
    fn literal_str(s: &str) -> String {
        let s = s.replace('\\', "\\\\").replace('\"', "\\\"");
        format!("\"{}\"", s)
    }

    // Build const array code
    let repo_body = format!(
"// This file is @generated automatically by build.rs; do not edit manually.

use crate::domain::country::iso_country::IsoCountryStatic;

/// All ISO country rows statically loaded at compile time
pub const ISO_COUNTRIES: &[IsoCountryStatic] = &[
{countries}
];

/// Static index on country_code
pub fn by_code(code: i32) -> Option<&'static IsoCountryStatic> {{
    ISO_COUNTRIES.iter().find(|c| c.country_code == code)
}}

pub fn by_alpha2(alpha2: &str) -> Option<&'static IsoCountryStatic> {{
    ISO_COUNTRIES.iter().find(|c| c.country_alpha2 == alpha2)
}}

pub fn by_alpha3(alpha3: &str) -> Option<&'static IsoCountryStatic> {{
    ISO_COUNTRIES.iter().find(|c| c.country_alpha3 == alpha3)
}}
"
    );

    fs::write(&static_out_path, repo_body)
        .expect("Unable to write iso_country_static_gen.rs");
}