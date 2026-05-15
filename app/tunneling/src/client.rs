use crate::config::ServerConfig;
use std::net::Ipv4Addr;
use std::process::Stdio;
use tokio::process;
use tokio::process::Command;

pub(super) struct Client {
    key_path: String,
    account: String,
    tunnel_ip: Ipv4Addr,
}

impl Client {
    pub fn new(key_path: String, account: String, tunnel_ip: Ipv4Addr) -> Self {
        Self {
            key_path,
            account,
            tunnel_ip,
        }
    }

    pub async fn connect<T>(&self, config: &T) -> anyhow::Result<TunnelSession>
    where
        T: ServerConfig + ?Sized,
    {
        let child = Command::new("ssh")
            .args([
                "-N",
                "-o",
                "ExitOnForwardFailure=yes",
                "-o",
                "ServerAliveInterval=60",
                "-i",
                self.key_path.as_str(),
                "-L",
                config.connect_string().as_str(),
                format!("{}@{}", self.account, self.tunnel_ip).as_str(),
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        Ok(TunnelSession { child })
    }
}

pub(super) struct TunnelSession {
    child: process::Child,
}

impl TunnelSession {
    pub async fn shutdown(mut self) -> anyhow::Result<()> {
        self.child.kill().await?;
        self.child.wait().await?;
        Ok(())
    }
}
