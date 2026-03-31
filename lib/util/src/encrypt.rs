use crate::byte::to_byte;
use aes::cipher::BlockDecryptMut;
use aes::Aes256;
use anyhow::anyhow;
use cbc::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use std::env;

#[derive(Debug)]
pub struct EncryptKey {
    key: [u8; 32],
    iv: [u8; 16],
}

impl EncryptKey {
    pub fn new_env(env_name: &str) -> anyhow::Result<Self> {
        let env = env::var(env_name)?;
        Ok(Self::new(env.as_str()))
    }

    pub const fn new(str: &str) -> Self {
        let key = to_byte::<32, 65>(&str);
        let iv = to_byte::<16, 65>(&str);

        Self { key, iv }
    }

    pub fn encrypt_cbc_pkcs7(&self, plain_text: String) -> Vec<u8> {
        let Self { key, iv } = self;
        Encryptor::<Aes256>::new(key.into(), iv.into())
            .encrypt_padded_vec_mut::<Pkcs7>(plain_text.as_bytes())
    }

    pub fn decrypt_cbc_pkcs7(&self, cipher_text: &[u8]) -> anyhow::Result<String> {
        let Self { key, iv } = self;
        let decrypted = Decryptor::<Aes256>::new(key.into(), iv.into())
            .decrypt_padded_vec_mut::<Pkcs7>(cipher_text)
            .map_err(|e| anyhow!("decrypt failed err: {}", e))?;
        Ok(String::from_utf8(decrypted)?)
    }
}