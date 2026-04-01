use anyhow::anyhow;
use base64::Engine;
use base64::engine::general_purpose;
use bytes::Bytes;

pub const fn to_byte<const N: usize, const D: u8>(input: &str) -> [u8; N] {
    let mut result = [D; N];
    let bytes = input.as_bytes();
    let len = if bytes.len() < N { bytes.len() } else { N };

    let mut i = 0;
    while i < len {
        result[i] = bytes[i];
        i += 1;
    }

    result
}

pub trait ToBase64 {
    fn to_base64(&self) -> String;
}

impl ToBase64 for [u8] {
    fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self)
    }
}

pub trait FromBase64 {
    fn from_base64(&self) -> anyhow::Result<Vec<u8>>;
}

impl FromBase64 for String {
    fn from_base64(&self) -> anyhow::Result<Vec<u8>> {
        general_purpose::STANDARD
            .decode(self)
            .map_err(|e| anyhow!("decode failed err: {}", e))
            .map(|v| v)
    }
}

pub trait ToJson {
    fn deserialize<T>(&self) -> anyhow::Result<T>
        where T: serde::de::DeserializeOwned;
}

impl ToJson for Bytes {
    fn deserialize<T>(&self) -> anyhow::Result<T>
    where
      T: serde::de::DeserializeOwned,
    {
        Ok(serde_json::from_slice(&self.as_ref())?)
    }
}