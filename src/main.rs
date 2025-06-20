use anyhow::anyhow;
use mimalloc::MiMalloc;

use crate::domain::country::service::country_service::CountryService;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod build_info;
pub mod domain;
pub mod state;

fn main() -> anyhow::Result<()> {
    let app_start_time = tokio::time::Instant::now();

    // main() equivalent
    let body = async {
        // install TLS provider
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .map_err(|e| anyhow!("Could not install TLS provider: {:?}", e))?;

        // on cloud, rely on the cloud service's env-variable injectors
        if std::env::var("IS_CLOUD").is_err() {
            dotenvy::dotenv().map_err(|e| anyhow::anyhow!("Failed to load .env: {}", e))?;
        }

        Ok(())
    };

    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(body);
}
