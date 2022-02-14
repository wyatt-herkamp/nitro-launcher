use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "type")]
    pub release_type: VersionType,
    pub url: String,
    pub id: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VersionType {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}