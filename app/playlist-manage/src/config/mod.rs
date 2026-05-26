use std::env;

#[derive(Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub port: u16
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self{
            client_id: env::var("GOOGLE_CLIENT_KEY").map_err(|e| anyhow::anyhow!("miss env GOOGLE_CLIENT_KEY err: {}", e))?,
            client_secret: env::var("GOOGLE_SECRET_KEY").map_err(|e| anyhow::anyhow!("miss env GOOGLE_SECRET_KEY err: {}", e))?,
            port: env::var("PORT")
              .unwrap_or_else(|_| "5000".to_owned())
              .parse()?,
        })
    }
}