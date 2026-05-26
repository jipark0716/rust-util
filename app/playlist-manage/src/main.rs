use actix_web::{web, App, HttpServer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::Config;

pub mod config;
pub mod api;

#[actix_web::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    registry()
      .with(fmt::layer())
      .with(LevelFilter::INFO)
      .init();

    let config = Config::from_env()?;

    HttpServer::new(move || {
        App::new()
          .configure(api::routes)
    })
      .bind(("0.0.0.0", config.port))?
      .run()
      .await?;
      
    Ok(())
}