use std::{env, fs, path::Path};

use chrono::Utc;
use postgres::{Client, NoTls};

fn main() {
    dotenvy::dotenv().ok();

    let out_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");
    let pkg_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME not found");
    let pkg_version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not found");

    // Get build time in RFC3339 format
    let build_time = Utc::now().to_rfc3339();

    // Get DB info from .env variables and assemble the DB URL
    let db_host = env::var("DB_HOST").expect("DB_HOST not found");
    let db_port = env::var("DB_PORT").expect("DB_PORT not found");
    let db_username = env::var("DB_USERNAME").expect("DB_USERNAME not found");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found");
    let db_name = env::var("DB_NAME").expect("DB_NAME not found");

    // Fix: use host=/tmp format for unix domain sockets if DB_HOST looks like an absolute path
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

    // We'll fetch the version of the Postgres backend as an example
    let row = client
        .query_one("SELECT version()", &[])
        .expect("Failed to execute version query");
    let db_version: String = row.get(0);

    let dest_path = Path::new(&out_dir).join("src/build_info.rs");
    let contents = format!(
        "pub const PKG_NAME: &str = \"{}\";\npub const PKG_VERSION: &str = \"{}\";\npub const BUILD_TIME: &str = \"{}\";\npub const DB_VERSION: &str = r#\"{}\"#;\n",
        pkg_name, pkg_version, build_time, db_version
    );

    fs::write(&dest_path, contents).expect("Unable to write build_info.rs");
}
