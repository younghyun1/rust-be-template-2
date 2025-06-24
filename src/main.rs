use anyhow::anyhow;
use mimalloc::MiMalloc;
use tracing::info;

use crate::{
    domain::country::service::country_service::CountryService,
    state::{server_env_config::ServerEnvConfig, server_state::ServerState},
    utility::tracing::setup_tracing::setup_logging,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod build_info;
pub mod domain;
pub mod state;
pub mod utility;

fn main() -> anyhow::Result<()> {
    let app_start_time = tokio::time::Instant::now();
    let _logger_raii_guard = setup_logging();

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

        let server_env_config = ServerEnvConfig::new_from_env()?;
        let server_state = ServerState::from_config(server_env_config).await;

        let mut conn = server_state.get_conn().await?;
        let mut country_service = CountryService::new_static();

        tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

        let mut times = Vec::with_capacity(200_000);
        for _ in 0..200_000 {
            let start = tokio::time::Instant::now();
            let _country = country_service.get_country(4)?;
            let elapsed = start.elapsed();
            times.push(elapsed);
        }
        times.sort();
        for t in &times {
            info!("{:?}", t);
        }

        drop(country_service);

        Ok(())
    };

    return tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(body);
}
