use std::{
    collections::HashMap,
    env,
    fs::OpenOptions,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use temp_dir::TempDir;
use thiserror::Error;
use tracing::{debug, error};
static JAVA_CHECK_BINARY: &[u8] = include_bytes!(env!("JAVA_CHECK"));
pub(crate) static JAVA_CHECK_LOCATION: OnceCell<TempDir> = OnceCell::new();
fn java_check_dir() -> Result<PathBuf, JavaUtilError> {
    if let Some(directory) = JAVA_CHECK_LOCATION.get().map(|v| v.clone()) {
        return Ok(directory.path().join("JavaUtilsCheck.jar"));
    }
    let directory = TempDir::new()?;
    let jar_file = directory.path().join("JavaUtilsCheck.jar");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&jar_file)?;
    file.write_all(JAVA_CHECK_BINARY)?;
    let _ = JAVA_CHECK_LOCATION.set(directory);

    Ok(jar_file)
}
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct JavaInstalls {
    pub default_java: Option<PathBuf>,
    pub installs: Vec<JavaConfiguration>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JavaConfiguration {
    pub path: PathBuf,
    pub home_path: PathBuf,
    pub vendor: String,
    pub version: String,
    pub java_major_release: String,
    pub java_type: JavaType,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum JavaType {
    JDK,
    JRE,
}
impl JavaType {
    #[cfg(not(windows))]
    pub fn get_java_type(path: impl AsRef<Path>) -> JavaType {
        if path.as_ref().join("bin").join("javac").exists() {
            JavaType::JDK
        } else {
            JavaType::JRE
        }
    }
    #[cfg(windows)]
    pub fn get_java_type(path: impl AsRef<Path>) -> JavaType {
        if path.as_ref().join("bin").join("javac.exe").exists() {
            JavaType::JDK
        } else {
            JavaType::JRE
        }
    }
}
#[derive(Debug, Error)]
pub enum JavaUtilError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Which(#[from] which::Error),
}

pub fn find_java() -> Result<Vec<JavaConfiguration>, JavaUtilError> {
    let mut paths = Vec::with_capacity(1);
    if let Some(bin) = std::env::var_os("JAVA_HOME")
        .map(PathBuf::from)
        .map(|v| v.join("bin").join("java"))
    {
        if bin.exists() {
            paths.push(bin)
        }
    }
    for java in which::which_all("java")? {
        paths.push(java);
    }

    let mut result = Vec::new();
    let save: PathBuf = java_check_dir()?;

    for path in paths {
        let Some(home) = path
            .parent()
            .and_then(|v| v.parent())
            .map(|v| v.to_path_buf())
        else {
            continue;
        };

        let java_type = JavaType::get_java_type(&home);
        debug!("Checking Java {path:?} with location {save:?}");
        // Make sure it did not get corrupted
        let mut command = Command::new(&path)
            .arg("-jar")
            .arg(save.as_os_str())
            .output()?;

        let mut output = output_to_map(String::from_utf8_lossy(&command.stdout).into_owned());
        command.stdout.flush()?;
        debug!("Got Result of {output:#?}");
        // TODO Java Version
        result.push(JavaConfiguration {
            path,
            home_path: home,
            java_major_release: output
                .remove("java.specification.version")
                .unwrap_or_default(),
            java_type: java_type,
            vendor: output.remove("java.vendor").unwrap_or_default(),
            version: output.remove("java.version").unwrap_or_default(),
        })
    }
    Ok(result)
}
fn output_to_map(output: String) -> HashMap<String, String> {
    let lines = output.lines();
    let mut result = HashMap::new();
    for line in lines {
        let property: Vec<&str> = line.split("=").collect();
        if property.len() != 2 {
            error!("Invalid Output {property:?}");
            continue;
        }
        let key = property[0].to_owned();
        let value = property[1].to_owned();
        result.insert(key, value);
    }
    result
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use anyhow::Context;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

    use crate::java_check_dir;

    #[test]
    pub fn test_save() -> anyhow::Result<()> {
        let save = java_check_dir()?;
        println!("Jar saved to {:?}", save);
        let java = which::which("java").expect("JAVA NO INSTALLED");
        // Make sure it did not get corrupted
        let _ = Command::new(java)
            .arg("-jar")
            .arg(save.as_os_str())
            .spawn()?;
        Ok(())
    }

    #[test]
    pub fn find_java() -> anyhow::Result<()> {
        let stdout_log = tracing_subscriber::fmt::layer().pretty();
        tracing_subscriber::registry()
            .with(stdout_log.with_filter(
                filter::Targets::new().with_targets([("nl_java_utils", LevelFilter::TRACE)]),
            ))
            .init();
        let save = super::find_java().with_context(|| "Could not locate java")?;
        println!("{save:#?}");
        Ok(())
    }
}
