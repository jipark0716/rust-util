use crate::core;
use crate::core::newtype::AccessToken;
use crate::core::newtype::{AppKey, AppSecret, GrantType};
use util::byte::ToJson;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Token {
    access_token: AccessToken,
}

#[derive(Debug, serde::Serialize)]
struct TokenRequest {
    app_key: AppKey,
    app_secret: AppSecret,
    grant_type: GrantType,
}

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait TokenProvider {
    async fn get_token(&self, key: AppKey, secret: AppSecret) -> anyhow::Result<Token>;
}

#[async_trait::async_trait]
impl<T: core::Client + Sync> TokenProvider for T {
    async fn get_token(&self, key: AppKey, secret: AppSecret) -> anyhow::Result<Token> {
        let response = self
            .post(
                "/oauth2/tokenP",
                TokenRequest {
                    app_key: key,
                    app_secret: secret,
                    grant_type: GrantType::ClientCredentials,
                },
            )
            .await?;

        let core::ResponseMessage {
            content,
            response_code,
        } = response;

        if response_code != 200 {
            return Err(anyhow::anyhow!(
                "failed to get token: {}",
                String::from_utf8(content.to_vec())?
            ));
        }

        let token = content.deserialize::<Token>()?;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{MockClient, ResponseMessage};
    use bytes::Bytes;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn success_get_token() {
        let client = MockClient::new(
            vec![ResponseMessage {
                content: Bytes::from(
                    r#"{
    "access_token": "token_text",
    "access_token_token_expired": "2026-04-02 11:49:36",
    "token_type": "Bearer",
    "expires_in": 86400
}"#,
                ),
                response_code: StatusCode::OK,
            }]
            .into(),
        );

        let token_result = client
            .get_token(AppKey("".to_string()), AppSecret("".to_string()))
            .await;

        assert!(
            token_result.is_ok(),
            "token_result should be an error {}",
            token_result.err().unwrap()
        );

        let token = token_result.unwrap();

        assert_eq!(
            token.access_token,
            AccessToken("token_text".to_string()),
            "invalid token: {:?}",
            token.access_token
        );
    }
}
