use std::env;
use anyhow::Context;
use sqlx::{Connection, FromRow};
use sqlx::mysql::MySqlConnection;

// tb_server_tunnel
#[derive(Debug, FromRow)]
pub(super) struct ServerTunnelEntity {
    tunnel_port: u16,
    dest_host: String,
    dest_port: u16,
}

impl ServerConfig for ServerTunnelEntity {
    fn connect_string(&self) -> String {
        format!(
            "127.0.0.1:{}:{}:{}",
            self.tunnel_port,
            self.dest_host,
            self.dest_port,
        )
    }
}

pub(super) async fn get_config() -> anyhow::Result<Vec<ServerTunnelEntity>> {
    let gateway_user = env::var("GATEWAY_USER").with_context(|| "GATEWAY_USER is not set")?;
    let gateway_pass = env::var("GATEWAY_PASS").with_context(|| "GATEWAY_PASS is not set")?;

    let mut connection = MySqlConnection::connect(format!("mysql://{gateway_user}:{gateway_pass}@127.0.0.1:10000").as_str())
      .await?;

    let configs = sqlx::query_as::<_, ServerTunnelEntity>(
        r#"
        select
            tunnel_port,
            dest_host,
            dest_port
        from bm_team.tb_server_tunnel
        "#,
    )
      .fetch_all(&mut connection)
      .await?;

    Ok(configs)
}

pub(super) trait ServerConfig {
    fn connect_string(&self) -> String;
}

pub(super) struct ServerDomainConfig {
    tunnel_port: u16,
    dest_domain: String,
    dest_port: u16,
}

impl ServerDomainConfig {
    pub(super) fn new(source_port: u16, dest_domain: String, dest_port: u16) -> Self {
        Self {
            tunnel_port: source_port,
            dest_domain,
            dest_port,
        }
    }
}

impl ServerConfig for ServerDomainConfig {
    fn connect_string(&self) -> String {
        format!(
            "127.0.0.1:{}:{}:{}",
            self.tunnel_port,
            self.dest_domain,
            self.dest_port,
        )
    }
}