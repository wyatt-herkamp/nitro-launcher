use std::{fmt::Debug, path::PathBuf};
mod manager;
use crate::{Component, Mod, NLCoreError};
use chrono::{DateTime, Duration, FixedOffset, Local};
pub use manager::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Instance<T: InstanceType> {
    pub name: String,
    pub uuid: Uuid,
    pub instance_type: T,
    pub java: PathBuf,
    pub play_time: PlayTimes,
    pub created_on: DateTime<FixedOffset>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayTimes {
    #[serde(with = "crate::duration")]
    pub total: Duration,
    pub play_times: Vec<PlayTime>,
}
impl<T: InstanceType> Instance<T> {
    pub fn get_last_play(&self) -> DateTime<FixedOffset> {
        let mut last_play = self.created_on;
        for play in &self.play_time.play_times {
            if play.ended_at > last_play {
                last_play = play.ended_at
            }
        }
        last_play
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayTime {
    pub started_at: DateTime<FixedOffset>,
    pub ended_at: DateTime<FixedOffset>,
}

pub trait InstanceType: Debug {
    fn type_name(&self) -> &str;
    fn get_minecraft_version(&self) -> &str;
    fn components(&self) -> &[Component];
    fn mods(&self) -> &[Mod];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vanilla {
    pub minecraft_version: String,
}

impl InstanceType for Vanilla {
    fn get_minecraft_version(&self) -> &str {
        &self.minecraft_version
    }

    fn mods(&self) -> &[Mod] {
        &[]
    }

    fn components(&self) -> &[Component] {
        &[]
    }

    fn type_name(&self) -> &str {
        "Vanilla Minecraft"
    }
}

pub struct CustomLaunch {
    pub minecraft_version: String,
    pub pre_launch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceDescription {
    pub name: String,
    pub type_name: String,
    pub last_played: DateTime<FixedOffset>,
    pub total_play_time: i64,
}
impl<'a, T: InstanceType> TryFrom<&'a Instance<T>> for InstanceDescription {
    type Error = NLCoreError;

    fn try_from(value: &'a Instance<T>) -> Result<Self, Self::Error> {
        Ok(InstanceDescription {
            name: value.name.clone(),
            type_name: value.instance_type.type_name().to_owned(),
            last_played: value.get_last_play(),
            total_play_time: value.play_time.total.num_seconds(),
        })
    }
}
