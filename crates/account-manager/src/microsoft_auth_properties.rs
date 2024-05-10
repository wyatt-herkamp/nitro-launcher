use std::{borrow::Cow, fs::read_to_string, io, path::Path};

use once_cell::sync::OnceCell;
use serde::Deserialize;
use tracing::error;

pub(crate) static AZURA_MICROSOFT_CLIENT: Option<&str> = option_env!("AZURA_MICROSOFT_CLIENT");

fn get_azura_microsoft() -> Option<Cow<'static, str>> {
    let env = std::env::var("AZURA_MICROSOFT_CLIENT").ok().map(Cow::Owned);
    env.or(AZURA_MICROSOFT_CLIENT.map(Cow::Borrowed))
}
