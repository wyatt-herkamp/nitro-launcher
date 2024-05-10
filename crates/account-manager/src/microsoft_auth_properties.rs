use std::{borrow::Cow, fs::read_to_string, io, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use tracing::error;

pub(crate) static AZURA_MICROSOFT_CLIENT: Option<&str> = option_env!("AZURA_MICROSOFT_CLIENT");
pub(crate) static MICROSOFT_VALUE: Option<&str> = option_env!("MICROSOFT_VALUE");
pub(crate) static SECRET_ID: Option<&str> = option_env!("SECRET_ID");
#[derive(Deserialize, Debug)]
struct LoginProperties {
    
}
pub static FILE_ENV: OnceCell<LoginProperties> = OnceCell::new();

pub fn load_file(file: impl AsRef<Path>) -> Result<(), io::Error> {
    let content = read_to_string(file.as_ref())?;
    let toml: LoginProperties =
        toml::from_str(&content).map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    if FILE_ENV.set(toml).is_err() {
        error!("The Login Properties has already been set!!");
    }
    Ok(())
}

fn get_azura_microsoft() -> Option<Cow<'static, str>> {
    let env = std::env::var("AZURA_MICROSOFT_CLIENT").ok().map(Cow::Owned);
    env.or(AZURA_MICROSOFT_CLIENT.map(Cow::Borrowed))
}
fn get_microsoft_value() -> Option<Cow<'static, str>> {
    let env = std::env::var("MICROSOFT_VALUE").ok().map(Cow::Owned);
    env.or(AZURA_MICROSOFT_CLIENT.map(Cow::Borrowed))
}
fn get_secret_id() -> Option<Cow<'static, str>> {
    let env = std::env::var("SECRET_ID").ok().map(Cow::Owned);
    env.or(AZURA_MICROSOFT_CLIENT.map(Cow::Borrowed))
}
