use reqwest::StatusCode;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinecraftAPIError {
    #[error("HTTP Error Code '{0}'")]
    HTTPError(StatusCode),
    #[error("Reqwest had an Error {0}")]
    ReqwestError(reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    JSONError(serde_json::Error),
    #[error("The Token has expired")]
    ExpiredToken,
    #[error("Internal Error {0}")]
    Custom(String),
}

impl From<reqwest::Error> for MinecraftAPIError {
    fn from(err: reqwest::Error) -> MinecraftAPIError {
        MinecraftAPIError::ReqwestError(err)
    }
}



impl From<serde_json::Error> for MinecraftAPIError {
    fn from(err: serde_json::Error) -> MinecraftAPIError {
        MinecraftAPIError::JSONError(err)
    }
}