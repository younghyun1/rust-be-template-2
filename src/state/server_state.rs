use anyhow::Result;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::bb8::{Pool, PooledConnection},
};

use crate::state::server_env_config::ServerEnvConfig;

pub struct ServerState {
    pool: Pool<AsyncPgConnection>,
}

impl ServerState {
    pub async fn from_config(config: ServerEnvConfig) -> Self {
        let pool = config
            .get_pool(num_cpus::get_physical() as u32)
            .await
            .unwrap_or_else(|e| panic!("Couldn't init connection pool! Error: {:?}", e));

        Self { pool }
    }

    pub fn pool(&self) -> &Pool<AsyncPgConnection> {
        &self.pool
    }

    pub async fn get_conn(&self) -> Result<PooledConnection<'_, AsyncPgConnection>> {
        Ok(self.pool.get().await?)
    }
}
