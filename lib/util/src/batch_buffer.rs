use std::time::Duration;
use tokio::pin;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tracing::instrument;

#[async_trait::async_trait]
pub trait BatchFlusher {
    type Input;
    async fn flush(&self, batch: Vec<Self::Input>) -> anyhow::Result<()>;
}

#[instrument(skip(flusher))]
pub fn create_batch_buffer<T, F>(flusher: F) -> (mpsc::Sender<T>, JoinHandle<()>)
where
    F: BatchFlusher<Input = T> + Send + Sync + 'static,
    T: Send + 'static,
{
    let (wr, rd) = mpsc::channel::<T>(1024 * 32);
    let handle = tokio::spawn(async move {
        let stream = ReceiverStream::new(rd).chunks_timeout(4, Duration::from_millis(300));

        pin!(stream);

        while let Some(batch) = stream.next().await {
            let batch_count = batch.len();
            if let Err(e) = flusher.flush(batch).await {
                tracing::error!("flush failed err: {}", e);
            } else {
                tracing::info!("flush end: {}", batch_count);
            }
        }

        tracing::info!("end flush type: {}", std::any::type_name::<T>());
    });

    (wr, handle)
}
