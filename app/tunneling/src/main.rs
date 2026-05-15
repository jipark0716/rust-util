mod client;
mod config;

use std::env;
use std::net::Ipv4Addr;
use anyhow::Context;
use futures::future::{join_all, try_join_all};
use tokio::signal;
use crate::config::{ServerDomainConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tunnel_key_path = env::var("TUNNEL_KEY_PATH").with_context(|| "TUNNEL_KEY_PATH is not set")?;

    let client = client::Client::new(
        tunnel_key_path.to_string(),
        "nick".to_string(),
        Ipv4Addr::new(172, 31, 14, 179),
    );

    let gateway_config = ServerDomainConfig::new(
        10000,
        "test-cluster8.cluster-ro-cghtjgokrsze.ap-northeast-2.rds.amazonaws.com".to_string(),
        56375,
    );

    let gateway_session = client.connect(&gateway_config).await?;

    // gateway 터널 연결 준비 대기
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let configs = config::get_config().await.context("get config failed")?;

    gateway_session.shutdown().await?;

    let connections = try_join_all(
        configs.iter().map(|config| {
            client.connect(config)
        })
    ).await?;

    signal::ctrl_c().await?;

    join_all(
        connections.into_iter().map(|connection| async move {
            let _ = connection.shutdown().await;
        })
    ).await;

    Ok(())
}