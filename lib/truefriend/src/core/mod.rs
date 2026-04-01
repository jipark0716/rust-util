mod client;
pub mod newtype;

pub use client::Client;
pub use client::ResponseMessage;

#[cfg(test)]
pub(crate) use client::mock::MockClient;