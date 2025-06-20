use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use std::{
    env,
    net::{IpAddr, ToSocketAddrs},
    path::PathBuf,
    time::Duration,
};

use diesel_async::{AsyncPgConnection, pooled_connection::bb8::Pool};
use uuid::Uuid;

pub struct ServerEnvConfig {
    // Database connection parameters
    db_host: String,
    db_port: u16,
    db_username: String,
    db_password: String,
    db_name: String,

    // AWS SES SMTP configuration
    aws_ses_smtp_url: String,
    aws_ses_smtp_username: String,
    aws_ses_smtp_access_key: String,

    // Server binding
    host_ip: IpAddr,
    host_port: u16,

    // TLS certificate and key paths
    cert_chain_dir: PathBuf,
    priv_key_dir: PathBuf,

    // Application identifier
    app_name_version: String,

    // API Key for internal authentication/middleware
    x_api_key: Uuid,

    // Running in AWS
    is_cloud: bool,

    // Frontend assets directory
    fe_assets_dir: String,
}

impl ServerEnvConfig {
    pub fn new_from_env() -> anyhow::Result<Self> {
        let db_port = env::var("DB_PORT")?
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Failed to parse DB_PORT: {}", e))?;
        let host_port = env::var("HOST_PORT")?
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Failed to parse HOST_PORT: {}", e))?;

        let is_cloud = match env::var("IS_CLOUD") {
            Ok(val) => val == "true" || val == "1",
            Err(_) => false,
        };

        Ok(ServerEnvConfig {
            db_host: env::var("DB_HOST")?,
            db_port,
            db_username: env::var("DB_USERNAME")?,
            db_password: env::var("DB_PASSWORD")?,
            db_name: env::var("DB_NAME")?,
            aws_ses_smtp_url: env::var("AWS_SES_SMTP_URL")?,
            aws_ses_smtp_username: env::var("AWS_SES_SMTP_USERNAME")?,
            aws_ses_smtp_access_key: env::var("AWS_SES_SMTP_ACCESS_KEY")?,
            host_ip: parse_ip_addr(env::var("HOST_IP")?)?,
            host_port,
            cert_chain_dir: env::var("CERT_CHAIN_DIR")?.into(),
            priv_key_dir: env::var("PRIV_KEY_DIR")?.into(),
            app_name_version: env::var("APP_NAME_VERSION")?,
            x_api_key: env::var("X_API_KEY")?.parse::<Uuid>()?,
            is_cloud,
            fe_assets_dir: env::var("FE_ASSETS_DIR")?,
        })
    }

    pub async fn get_pool(&self, num_cores: u32) -> anyhow::Result<Pool<AsyncPgConnection>> {
        let db_url = if self.db_host.starts_with('/') {
            format!(
                "postgresql://{}:{}@{}/{}?host={}&port={}",
                self.db_username,
                self.db_password,
                "localhost",
                self.db_name,
                self.db_host,
                self.db_port
            )
        } else {
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                self.db_username, self.db_password, self.db_host, self.db_port, self.db_name
            )
        };

        let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);

        let pool = Pool::builder()
            .min_idle(Some(num_cores))
            .max_size(num_cores * 10)
            .connection_timeout(Duration::from_secs(2))
            .build(manager)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to build connection pool: {}", e))?;

        Ok(pool)
    }
}

pub fn parse_ip_addr<S: AsRef<str>>(s: S) -> anyhow::Result<IpAddr> {
    let ip_str = s.as_ref().trim();

    // Try basic parse first
    if let Ok(ip) = ip_str.parse::<IpAddr>() {
        return Ok(ip);
    }

    // Handle IPv6 addresses in bracket notation, e.g. "[::1]"
    if ip_str.starts_with('[') && ip_str.ends_with(']') {
        let inside = &ip_str[1..ip_str.len() - 1];
        if let Ok(ipv6) = inside.parse::<std::net::Ipv6Addr>() {
            return Ok(IpAddr::V6(ipv6));
        }
    }

    // Attempt to resolve hostnames (including localhost) if standard parse fails
    // (Common when users specify HOST_IP as 'localhost', 'ip6-localhost', etc.)
    if let Ok(mut addrs) = (ip_str, 0).to_socket_addrs() {
        if let Some(addr) = addrs.find_map(|sa| Some(sa.ip())) {
            return Ok(addr);
        }
    }

    // Special-case: allow "localhost" or "ip6-localhost"
    match ip_str {
        "localhost" => return Ok(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))),
        "ip6-localhost" | "ip6-loopback" => return Ok(IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)),
        _ => {}
    }

    Err(anyhow::anyhow!(
        "Could not parse '{}' as a valid IP address (IPv4 or IPv6)",
        ip_str
    ))
}
