use anyhow::anyhow;
use clickhouse::insert::Insert;
use clickhouse::Row;

pub trait Entity: serde::Serialize + for<'de> serde::Deserialize<'de> {
    fn table_name() -> &'static str;
}

#[async_trait::async_trait]
pub trait EntityExtensions {
    async fn insertx<T>(&self) -> anyhow::Result<Insert<T>>
    where
        T: Entity + Send + clickhouse::Row + Sync;
}

#[async_trait::async_trait]
impl EntityExtensions for clickhouse::Client {
    async fn insertx<T>(&self) -> anyhow::Result<Insert<T>>
    where
      T: Entity + Send + Row + Sync
    {
        match self.insert::<T>(T::table_name()).await {
            Ok(o) => Ok(o),
            Err(e) => Err(anyhow!("insert failed err: {}", e)),
        }
    }
}
