mod source;
mod dest;
use clickhouse::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default()
      .with_url("http://localhost:8123")
      .with_user("admin")
      .with_password("password1234");

    let mut cursor = client
      .query("SELECT ?fields FROM send_log.sys_kakaotalk_log_prod WHERE reg_dt > ?")
      .bind("2026-06-01")
      .fetch::<source::Entity>()?;

    while let Some(row) = cursor.next().await? {
        println!("{:?}", row);
        return Ok(())
    }

    Ok(())
}