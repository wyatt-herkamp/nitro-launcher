use std::{
    fmt::Display,
    fs::{read_to_string, rename, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use account_types::{AccountsFile, VersionCheck};
use current_semver::current_major;

use thiserror::Error;
use tracing::warn;
mod account_types;
mod microsoft_auth_properties;
static CURRENT_MAJOR: u64 = current_major!() as u64;
#[derive(Debug, Error)]
pub enum AccountLoadError {
    #[error("Parameter {0} is not a directory")]
    SrcFolderIsNotADirectory(PathBuf),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not Parse Accounts File {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("Could Not Serialize Accounts file. THIS IS A BUG {0}")]
    TomlSer(#[from] toml::ser::Error),
}
pub struct AccountManager {
    pub path: PathBuf,
    pub accounts: AccountsFile,
}

impl AccountManager {
    pub async fn load_from_directory(
        dir: impl AsRef<Path>,
    ) -> Result<AccountManager, AccountLoadError> {
        let dir = dir.as_ref();
        if !dir.is_dir() {
            return Err(AccountLoadError::SrcFolderIsNotADirectory(
                dir.to_path_buf(),
            ));
        }
        let accounts_file = dir.join("accounts.toml");
        if !accounts_file.exists() {
            return Ok(AccountManager {
                path: accounts_file,
                accounts: AccountsFile::default(),
            });
        }
        let accounts_file_content = read_to_string(&accounts_file)?;
        let version_check: VersionCheck = toml::from_str(&accounts_file_content)?;
        let file_major = version_check.version.major;
        let accounts = if file_major > CURRENT_MAJOR {
            // If the file is on a newer version. Attempt to load the an older version
            // The old file will be immied
            let version_backup = get_version_backup_path(&dir, CURRENT_MAJOR);
            if version_backup.exists() {
                let accounts_file_content = read_to_string(&version_backup)?;
                toml::from_str(&accounts_file_content)?
            } else {
                AccountsFile::default()
            }
        } else if file_major < CURRENT_MAJOR {
            // If the current version is newer. Move the older version to a backup and create a new file
            let current_semver = current_semver::current_semver!();
            warn!("Accounts file is on version {}. Account Manager is on {}. This will result in the file being reset", version_check.version,current_semver);
            // Move accounts.toml to accounts.v{major}.bak.toml
            let version_backup = get_version_backup_path(&dir, file_major);
            rename(&accounts_file, version_backup)?;
            AccountsFile::default()
        } else {
            toml::from_str(&accounts_file_content)?
        };
        let manager = AccountManager {
            path: accounts_file,
            accounts,
        };
        manager.save()?;
        Ok(manager)
    }

    pub fn save(&self) -> Result<(), AccountLoadError> {
        let as_toml = toml::to_string_pretty(&self.accounts)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        file.write_all(as_toml.as_bytes())?;
        Ok(())
    }
}

fn get_version_backup_path(dir: &impl AsRef<Path>, major: impl DisplayMajor) -> PathBuf {
    dir.as_ref()
        .join(format!("accounts.v{}.bak.toml", major.display_major()))
}
trait DisplayMajor {
    fn display_major(&self) -> impl Display;
}
impl DisplayMajor for u64 {
    fn display_major(&self) -> impl Display {
        self
    }
}
impl DisplayMajor for usize {
    fn display_major(&self) -> impl Display {
        self
    }
}
impl DisplayMajor for semver::Version {
    fn display_major(&self) -> impl Display {
        self.major
    }
}
