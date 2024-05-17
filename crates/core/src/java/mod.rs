use std::{fs::read_to_string, path::PathBuf, sync::Arc};

use nl_java_utils::{JavaConfiguration, JavaInstalls};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::NLCoreError;
#[derive(Debug, Clone)]
pub struct JavaManager(Arc<InnerJavaManager>);

#[derive(Debug)]
pub struct InnerJavaManager {
    config: Mutex<JavaInstalls>,
    path: PathBuf,
}
impl JavaManager {
    pub fn new(app_dir: PathBuf) -> Result<Self, NLCoreError> {
        let java = app_dir.join("java.toml");
        if java.exists() {
            let toml = read_to_string(&java)?;
            let config: JavaInstalls = toml::from_str(&toml)?;
            return Ok(Self(Arc::new(InnerJavaManager {
                config: Mutex::new(config),
                path: java,
            })));
        }
        return Ok(Self(Arc::new(InnerJavaManager {
            config: Default::default(),
            path: java,
        })));
    }
}
