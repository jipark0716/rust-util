mod server;

use tonic::transport::Server;
use crate::server::HealthCheckService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:5000".parse()?;
    
    Server::builder()
        .add_service(HealthCheckService::new())
        .serve(addr)
        .await?;

    println!("Server running at https://{}", addr);

    Ok(())
}