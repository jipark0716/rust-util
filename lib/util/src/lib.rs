mod batch_buffer;
pub mod byte;
pub mod encrypt;
mod numeric;
pub mod error;
pub mod clickhouse_batch_buffer;

#[async_trait::async_trait]
pub trait Shutdown {
    async fn shutdown(self);
}