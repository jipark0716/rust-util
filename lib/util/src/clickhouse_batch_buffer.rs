use crate::batch_buffer::{BatchFlusher, create_batch_buffer};
use clickhouse::{RowOwned, RowWrite};
use entity::Entity;
use entity::EntityExtensions;
use std::marker::PhantomData;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

pub trait ClickhouseBatchExtensions {
    fn create_batch_buffer<T>(&self) -> (mpsc::Sender<T>, JoinHandle<()>)
    where
        T: Entity + Send + Sync + RowOwned;
}

impl ClickhouseBatchExtensions for clickhouse::Client {
    fn create_batch_buffer<T>(&self) -> (mpsc::Sender<T>, JoinHandle<()>)
    where
        T: Entity + Send + Sync + RowOwned,
    {
        create_batch_buffer(ClickhouseBatchFlusher::<T>::new(self.clone()))
    }
}

pub struct ClickhouseBatchFlusher<T> {
    client: clickhouse::Client,
    _marker: PhantomData<T>,
}

impl<T> ClickhouseBatchFlusher<T> {
    pub fn new(client: clickhouse::Client) -> Self {
        Self {
            client,
            _marker: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<T> BatchFlusher for ClickhouseBatchFlusher<T>
where
    T: Entity + Send + Sync + RowWrite + RowOwned,
{
    type Input = T;

    async fn flush(&self, batch: Vec<Self::Input>) -> anyhow::Result<()> {
        let mut insert = self.client.insertx::<Self::Input>().await?;

        for row in &batch {
            insert.write(row).await?;
        }

        insert.end().await?;

        Ok(())
    }
}
