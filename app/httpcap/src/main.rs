mod capture;

use tokio::sync::mpsc::Sender;
use actix_web::{App, HttpServer, web};
use clap::Parser;
use tracing_core::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, registry};
use once_cell::sync::Lazy;
use tokio::task::JoinHandle;
use util::{report};
use util::clickhouse_batch_buffer::ClickhouseBatchExtensions;
use crate::capture::HttpRequestEntity;

#[derive(Parser, Debug)]
struct Command {
    #[arg(long, default_value_t = 8000)]
    port: u16,

    #[arg(long)]
    host: String,

    #[arg(long)]
    user: String,

    #[arg(long)]
    password: String,

    #[arg(long)]
    database: String,
}

pub struct State {
    pub http_request_buffer: Sender<HttpRequestEntity>,
}

impl State {
    pub fn new() -> (Self, Vec<JoinHandle<()>>) {
        let clickhouse_client = clickhouse::Client::default()
          .with_url(ARGS.host.as_str())
          .with_user(ARGS.user.as_str())
          .with_password(ARGS.password.as_str())
          .with_database(ARGS.database.as_str());

        let (http_request_buffer, handle) = clickhouse_client.create_batch_buffer::<HttpRequestEntity>();

        (Self {
            http_request_buffer,
        }, vec![handle])
    }
}

static ARGS: Lazy<Command> = Lazy::new(|| Command::parse());

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    registry()
      .with(fmt::layer())
      .with(LevelFilter::INFO)
      .init();

    tracing::info!("start with ARGS: {:?}", &*ARGS);

    let (state, handles) = State::new();
    let state = web::Data::new(state);

    if let Err(e) = HttpServer::new(move || {
        App::new()
          .app_data(state.clone())
          .default_service(web::route().to(capture::capture))
    })
      .bind(("0.0.0.0", ARGS.port))?
      .run()
      .await
    {
        return Err(report!(e, "Failed to start server"));
    }


    tracing::info!("start state join");

    for handle in handles {
        if let Err(e) = handle.await {
            tracing::error!("join handle error: {:?}", e);
        }
    }

    tracing::info!("end state join");

    Ok(())
}