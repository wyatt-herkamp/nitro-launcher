use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
    sync::Arc,
};

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

use crate::NLCoreError;

use super::{Instance, InstanceDescription, InstanceType};

#[derive(Debug)]
pub struct InstanceManager<T: InstanceType>(Arc<Mutex<InnerInstanceManager<T>>>);
impl<T: InstanceType> Clone for InstanceManager<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: InstanceType> From<InnerInstanceManager<T>> for InstanceManager<T> {
    fn from(value: InnerInstanceManager<T>) -> Self {
        InstanceManager(Arc::new(Mutex::new(value)))
    }
}
impl<T: InstanceType + for<'de> Deserialize<'de> + Serialize> InstanceManager<T> {
    pub fn new(app_dir: PathBuf) -> Result<Self, NLCoreError> {
        assert!(
            !app_dir.is_dir(),
            "The provided app_dir is not a directory!"
        );

        let instance_dir = app_dir.join("instances");
        if !instance_dir.exists() {
            create_dir_all(&instance_dir)?;
        }

        let inner = InnerInstanceManager {
            instance_dir,
            instances: HashMap::new(),
        };
        Ok(inner.into())
    }

    pub fn load_instances(&self) -> Result<usize, NLCoreError> {
        let mut instances_loaded = 0;
        let mut instances: HashMap<PathBuf, Instance<T>> = HashMap::new();
        let base_dir = self.get_instance_dir();

        for dir in base_dir.read_dir()? {
            let entry = match dir {
                Ok(entry) => entry,
                Err(err) => {
                    error!("Could not read entry in {base_dir:?} error {err:?}");
                    continue;
                }
            };

            if !entry.path().is_dir() {
                debug!(
                    "Skipping Entry {:?} because it is not a directory",
                    entry.path()
                );
                continue;
            }
            let instance_file = entry.path().join("instance.toml");
            if !instance_file.exists() {
                debug!(
                    "Skipping Entry {:?} does not contain an instance.toml",
                    entry.path()
                );
                continue;
            }

            let read: String = read_to_string(&instance_file)?;

            let parsed: Instance<T> = match toml::from_str(&read) {
                Ok(ok) => ok,
                Err(err) => {
                    error!("Invalid Instance.toml. {err:?} in file {instance_file:?}");
                    continue;
                }
            };

            info!("Loaded Instance {} in {:?}", parsed.name, entry.path());
            instances_loaded = instances_loaded + 1;

            instances.insert(entry.path(), parsed);
        }
        {
            let mut locked = self.0.lock();
            locked.instances = instances
        }
        Ok(instances_loaded)
    }
    pub fn get_instance_descriptions(&self) -> Vec<InstanceDescription> {
        let locked = self.0.lock();
        let mut instances = Vec::with_capacity(locked.instances.len());
        for (_, instance) in locked.instances.iter() {
            let instance_desc = match InstanceDescription::try_from(instance) {
                Ok(ok) => ok,
                Err(err) => {
                    warn!("Unable to create an instance description for {instance:?} err {err:?}");
                    continue;
                }
            };
            instances.push(instance_desc)
        }

        instances
    }
    pub fn get_instance_dir(&self) -> PathBuf {
        self.0.lock().instance_dir.clone()
    }
}
#[derive(Debug)]
pub struct InnerInstanceManager<T: InstanceType> {
    pub instance_dir: PathBuf,
    pub instances: HashMap<PathBuf, Instance<T>>,
}
