use std::error::Error;
use serde::Serialize;
use tauri::{InvokeError, InvokeHandler};
use serde_json::Value as JsonValue;

use thiserror::Error;
use crate::minecraft;
use crate::minecraft::error::MinecraftAPIError;

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("JSON error {0}")]
    JSONError(serde_json::Error),
    #[error("Minecraft error {0}")]
    MinecraftAPI(minecraft::error::MinecraftAPIError),
    #[error("IO Error: {0}")]
    IOError(std::io::Error),
    #[error("Internal Error {0}")]
    Error(String),
}

impl Into<InvokeError> for InternalError {
    fn into(self) -> InvokeError {
        let result = serde_json::to_string(&self.to_string()).unwrap();
        result.into()
    }
}

impl From<MinecraftAPIError> for InternalError {
    fn from(error: MinecraftAPIError) -> Self {
        return InternalError::MinecraftAPI(error);
    }
}

impl From<std::io::Error> for InternalError {
    fn from(error: std::io::Error) -> Self {
        return InternalError::IOError(error);
    }
}

impl From<serde_json::Error> for InternalError {
    fn from(err: serde_json::Error) -> InternalError {
        InternalError::JSONError(err)
    }
}