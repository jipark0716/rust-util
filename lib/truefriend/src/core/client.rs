use reqwest::StatusCode;
use serde::Serialize;

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait Client {
    async fn post<T>(&self, uri: &str, body: T) -> anyhow::Result<ResponseMessage>
    where
        T: Serialize + Send;
}

pub struct ResponseMessage {
    pub content: bytes::Bytes,
    pub response_code: StatusCode,
}

mod client {
    use super::{Client, ResponseMessage};
    use anyhow::Context;
    use serde::Serialize;

    #[allow(dead_code)]
    pub struct HttpClient {
        client: reqwest::Client,
    }

    #[async_trait::async_trait]
    impl Client for HttpClient {
        async fn post<T>(&self, uri: &str, body: T) -> anyhow::Result<ResponseMessage>
        where
            T: Serialize + Send,
        {
            let response = self.client.post(uri).json(&body).send().await?;

            Ok(ResponseMessage {
                response_code: response.status(),
                content: response
                    .bytes()
                    .await
                    .context("failed to read response body")?,
            })
        }
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use super::{Client, ResponseMessage};
    use serde::Serialize;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    pub struct MockClient {
        stack: Mutex<VecDeque<ResponseMessage>>,
    }

    impl MockClient {
        pub fn new(mock_data: VecDeque<ResponseMessage>) -> Self {
            Self {
                stack: Mutex::new(mock_data),
            }
        }
    }

    #[async_trait::async_trait]
    impl Client for MockClient {
        async fn post<T>(&self, _: &str, _: T) -> anyhow::Result<ResponseMessage>
        where
            T: Serialize + Send,
        {
            let response = self
                .stack
                .lock()
                .map_err(|e| anyhow::anyhow!("failed to write to stack: {}", e))?
                .pop_front();

            Ok(response.ok_or_else(|| anyhow::anyhow!("no more responses in stack"))?)
        }
    }
}
