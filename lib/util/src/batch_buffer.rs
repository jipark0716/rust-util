// use std::time::Duration;
// use async_trait::async_trait;
// use tokio::pin;
// use tokio::sync::mpsc;
// use tokio::sync::mpsc::Sender;
// use tokio::task::JoinHandle;
// use tokio_stream::{wrappers::ReceiverStream, StreamExt};
// use crate::Shutdown;
// 
// pub enum FlushMessage<T> {
//     message(T),
//     close
// }
// 
// struct FlushShutdown<T> {
//     handle: JoinHandle<()>,
//     wr: Sender<FlushMessage<T>>,
// }
// 
// #[async_trait]
// impl <T> Shutdown for FlushShutdown<T> {
//     async fn shutdown(self) {
//         let Self {
//             handle,
//             wr,
//         } = self;
// 
//         for _ in 0..3 {
//             if let Some(err) = wr.send(FlushMessage::<T>::close).await.err() {
//                 // todo 에러 찍어라이
//             } else {
//                 break;
//             }
// 
//             tokio::time::sleep(Duration::from_millis(100)).await;
//         }
// 
//         // let wr.send(FlushMessage::<T>::close).await;
//         if let Some(err) = handle.await.err() {
//             // todo 에러 찍어라이
//         }
//     }
// }
// 
// pub fn new<T, F>(flusher: F) -> (Sender<FlushMessage<T>>, impl Shutdown)
// where
//     F: BatchFlusher<Input = T> + Send + 'static,
//     T: Send + 'static,
// {
//     let (wr, rd) = mpsc::channel::<FlushMessage<T>>(1024 * 32);
//     let handle = tokio::spawn(async move {
//         let stream = ReceiverStream::new(rd);
// 
//         let stream = stream
//             .chunks_timeout(1024, Duration::from_millis(50));
//         pin!(stream);
// 
//         let mut close_requested = false;
//         while let Some(batch) = stream.next().await && !close_requested {
//             let mut messages = Vec::with_capacity(batch.len());
// 
//             for item in batch {
//                 match item {
//                     FlushMessage::message(message) => messages.push(message),
//                     FlushMessage::close => close_requested = true,
//                 }
//             }
// 
//             if !messages.is_empty() {
//                 for _ in 0..3 {
//                     let a = flusher.flush(&messages).await;
//                     if flusher.flush(&messages).await.is_ok() {
//                         break;
//                     }
//                     tokio::time::sleep(Duration::from_millis(100)).await;
//                 }
//             }
//         }
//     });
// 
//     (
//         wr.clone(),
//         FlushShutdown {
//             handle,
//             // send_close_message: || ,
//         }
//     )
// }
// 
// #[async_trait]
// pub trait BatchFlusher {
//     type Input;
//     async fn flush(&self, batch: &[Self::Input]) -> anyhow::Result<()>;
// }
