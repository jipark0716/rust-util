use serde::{Serialize};

#[derive(Debug, serde::Serialize)]
pub struct AppKey(pub String);
#[derive(Debug, serde::Serialize)]
pub struct AppSecret(pub String);
#[derive(Debug, serde::Serialize, PartialEq, Eq, serde::Deserialize)]
pub struct AccessToken(pub String);
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    ClientCredentials,
}