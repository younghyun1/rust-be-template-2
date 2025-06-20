use std::{env, fs, path::Path};

use postgres::{Client, NoTls, Row};

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
    let static_out_path =
        Path::new(&out_dir).join("src/domain/country/repository/iso_country_static_gen.rs");

    let rows = client
        .query(
            "SELECT country_code, country_alpha2, country_alpha3, country_eng_name, country_currency, \
                    phone_prefix, country_flag, is_country, country_primary_language \
             FROM iso_country ORDER BY country_code",
            &[],
        )
        .expect("Failed to query iso_country");

    let repo_body = gen_country_static_code(&rows);
    fs::write(&static_out_path, repo_body).expect("Unable to write iso_country_static_gen.rs");
}

/// Formats a Rust 'static str literal (with proper escaping).
fn literal_str(s: &str) -> String {
    let s = s.replace('\\', "\\\\").replace('\"', "\\\"");
    format!("\"{}\"", s)
}

/// Generate Rust code for the static ISO country table and lookup PHF maps.
fn gen_country_static_code(rows: &[Row]) -> String {
    let mut countries = String::new();
    let mut code_keyvals = String::new();
    let mut alpha2_keyvals = String::new();
    let mut alpha3_keyvals = String::new();

    for (i, row) in rows.iter().enumerate() {
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
    }},
",
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

        code_keyvals.push_str(&format!(
            "{code}i32 => &ISO_COUNTRIES[{idx}],\n",
            code = country_code,
            idx = i
        ));
        alpha2_keyvals.push_str(&format!(
            "{alpha2} => &ISO_COUNTRIES[{idx}],\n",
            alpha2 = literal_str(&country_alpha2),
            idx = i
        ));
        alpha3_keyvals.push_str(&format!(
            "{alpha3} => &ISO_COUNTRIES[{idx}],\n",
            alpha3 = literal_str(&country_alpha3),
            idx = i
        ));
    }

    let n_countries = rows.len();

    format!(
        r#"// This file is @generated automatically by build.rs; do not edit manually.

use crate::domain::country::iso_country::IsoCountryStatic;
use phf::{{phf_map, Map}};

/// All ISO country rows statically loaded at compile time.
pub static ISO_COUNTRIES: [IsoCountryStatic; {len}] = [
{countries}
];

/// PHF static maps for fast lookup by code, alpha2, alpha3.
pub static BY_CODE: Map<i32, &'static IsoCountryStatic> = phf_map! {{
{code_keyvals}
}};

pub static BY_ALPHA2: Map<&'static str, &'static IsoCountryStatic> = phf_map! {{
{alpha2_keyvals}
}};

pub static BY_ALPHA3: Map<&'static str, &'static IsoCountryStatic> = phf_map! {{
{alpha3_keyvals}
}};

/// Lookup by numeric country code.
pub fn by_code(code: i32) -> Option<&'static IsoCountryStatic> {{
    BY_CODE.get(&code).copied()
}}

/// Lookup by country alpha-2 code.
pub fn by_alpha2(alpha2: &str) -> Option<&'static IsoCountryStatic> {{
    BY_ALPHA2.get(alpha2).copied()
}}

/// Lookup by country alpha-3 code.
pub fn by_alpha3(alpha3: &str) -> Option<&'static IsoCountryStatic> {{
    BY_ALPHA3.get(alpha3).copied()
}}
"#,
        len = n_countries,
        countries = countries,
        code_keyvals = code_keyvals,
        alpha2_keyvals = alpha2_keyvals,
        alpha3_keyvals = alpha3_keyvals
    )
}
