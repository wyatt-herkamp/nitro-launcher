use derive_more::From;
use minecraft_rs::{authentication::AccountSave, profile::ProfileResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, From)]
#[serde(tag = "type", content = "data")]
pub enum Account {
    Microsoft(MicrosoftAccountType),
    Offline(OfflineAccountType),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OfflineAccountType {
    profile: ProfileResponse,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MicrosoftAccountType {
    pub account_save: AccountSave,
    pub profile: ProfileResponse,
}
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct VersionCheck {
    pub version: semver::Version,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountsFile {
    pub version: semver::Version,

    pub accounts: Vec<Account>,
}
impl Default for AccountsFile {
    fn default() -> Self {
        Self {
            version: current_semver::current_semver!(),
            accounts: Default::default(),
        }
    }
}
#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use minecraft_rs::{authentication::{MicrosoftToken, MinecraftSave, XboxUserSave}, profile::ProfileResponse};
    use uuid::Uuid;

    use super::*;

    #[test]
    pub fn test_toml() -> anyhow::Result<()> {
        let test_accounts: Vec<Account> = vec![
            OfflineAccountType {
                profile: ProfileResponse {
                    id: Uuid::default(),
                    name: String::new(),
                    skins: Vec::new(),
                    capes: Vec::new(),
                },
            }
            .into(),
            MicrosoftAccountType {
                account_save: AccountSave {
                    microsoft_token: MicrosoftToken {
                        refresh_token: String::new(),
                    },
                    xbox: XboxUserSave {
                        expires: DateTime::default(),
                        token: String::new(),
                        user_hash: String::new(),
                    },
                    minecraft_save: MinecraftSave {
                        expires: DateTime::default(),
                        token: String::new(),
                    },
                },
                profile: ProfileResponse {
                    id: Uuid::default(),
                    name: String::new(),
                    skins: Vec::new(),
                    capes: Vec::new(),
                },
            }
            .into(),
        ];
        let accounts_file = AccountsFile {
            accounts: test_accounts,
            ..Default::default()
        };

        let serialized = toml::to_string_pretty(&accounts_file)?;

        let deserialized: AccountsFile = toml::from_str(&serialized)?;

        pretty_assertions::assert_eq!(accounts_file, deserialized);

        Ok(())
    }
}
