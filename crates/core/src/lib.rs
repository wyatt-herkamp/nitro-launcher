use std::io;

use thiserror::Error;
pub mod config;
pub mod duration;
pub mod instance;
pub mod java;
pub struct Mod {
    pub mod_name: String,
    pub enabled: bool,
    pub source: Option<String>,
}
pub struct Component {
    pub name: String,
    pub version: String,
}
#[derive(Debug, Error)]
pub enum NLCoreError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    JavaUtils(#[from] nl_java_utils::JavaUtilError),
    #[error("Could not Parse Accounts File {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Could Not Serialize Accounts file. THIS IS A BUG {0}")]
    TomlSer(#[from] toml::ser::Error),
}
