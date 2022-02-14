use reqwest::Client;
use crate::minecraft::error::MinecraftAPIError;
use crate::minecraft::structs::VersionManifest;

pub mod structs;
pub mod error;

pub struct MinecraftAPI;

impl MinecraftAPI {
    pub async fn get_version_manifest(client: &Client) -> Result<VersionManifest, MinecraftAPIError> {
        let request = client.get("https://launchermeta.mojang.com/mc/game/version_manifest.json").send().await?;
        let x = request.text().await?;
        let result: VersionManifest = serde_json::from_str(&*x)?;
        Ok(result)
    }
}