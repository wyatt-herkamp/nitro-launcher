use std::{
    borrow::{Borrow, Cow},
    fs::read_to_string,
    io,
    path::Path,
};

use once_cell::sync::{Lazy, OnceCell};
use serde::Deserialize;
use tracing::error;

pub(crate) static AZURA_MICROSOFT_CLIENT: Option<&str> = option_env!("AZURA_MICROSOFT_CLIENT");
pub(crate) static AUTH_API_USER_AGENT: Option<&str> = option_env!("AUTH_API_USER_AGENT");

pub(crate) fn get_azura_microsoft() -> Option<Cow<'static, str>> {
    let env = std::env::var("AZURA_MICROSOFT_CLIENT").ok().map(Cow::Owned);
    env.or(AZURA_MICROSOFT_CLIENT.map(Cow::Borrowed))
}

pub(crate) fn get_auth_api_user_agent() -> Cow<'static, str> {
    let env = std::env::var("AUTH_API_USER_AGENT").ok().map(Cow::Owned);
    let user_agent = env.or(AUTH_API_USER_AGENT.map(Cow::Borrowed));

    match user_agent {
        Some(ok) => ok,
        None => Cow::Borrowed("Nitro Launcher"),
    }
}
