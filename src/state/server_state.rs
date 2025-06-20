use anyhow::Result;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::bb8::{Pool, PooledConnection},
};

pub struct ServerState {
    pool: Pool<AsyncPgConnection>,
}

impl ServerState {
    pub fn pool(&self) -> &Pool<AsyncPgConnection> {
        &self.pool
    }

    pub async fn get_conn(&self) -> Result<PooledConnection<'_, AsyncPgConnection>> {
        Ok(self.pool.get().await?)
    }
}
