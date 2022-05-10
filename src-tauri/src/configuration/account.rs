use std::fs::{OpenOptions, read_dir};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::auth::Authentication;
use crate::InternalError;
use crate::utils::resources::get_nitro_launcher_dir;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub uuid: Uuid,
    pub authorization: Authentication,
}

pub fn get_accounts() -> Result<Vec<Account>, InternalError> {
    let mut accounts = Vec::new();
    let accounts_folder = get_nitro_launcher_dir().join("accounts");

    let result = read_dir(accounts_folder)?;
    for data in result {
        let file = OpenOptions::new().read(true).open(data?.path())?;
        let account: Account = serde_json::from_reader(file)?;
        accounts.push(account)
    }
    return Ok(accounts);
}