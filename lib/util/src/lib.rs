mod batch_buffer;
pub mod byte;
pub mod encrypt;
mod numeric;

#[async_trait::async_trait]
pub trait Shutdown {
    async fn shutdown(self);
}