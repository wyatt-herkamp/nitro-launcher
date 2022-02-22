#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use nitro_log::config::Config;
use nitro_log::NitroLogger;
use reqwest::{Client, ClientBuilder};
use tauri::http::Response;
use crate::error::InternalError;
use crate::minecraft::structs::VersionManifest;
use crate::utils::resources::Resources;

pub mod error;
pub mod minecraft;
pub mod utils;
pub mod news;
pub mod auth;

pub struct GeneralState {
    pub client: Client,
}

fn main() {
    let client = ClientBuilder::default().user_agent("Nitro Launcher by kingtux.me").build().expect("Unable to create Web Client");
    let state = GeneralState {
        client
    };


    let config: Config = serde_json::from_str(Resources::file_get_string("log/debug.json").as_str()).unwrap();
    NitroLogger::load(config, None).unwrap();
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_accounts, get_games, login,get_minecraft_versions])
        .register_uri_scheme_protocol("nitroLauncher", |a, h| {
            println!("{}", h.uri());
            return Ok(Response::default());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_accounts() -> Result<String, InternalError> {
    Ok("".to_string())
}

#[tauri::command]
fn get_games() -> Result<String, InternalError> {
    Ok("".to_string())
}

#[tauri::command]
fn login(account: String) -> Result<String, InternalError> {
    Ok("".to_string())
}

#[tauri::command]
async fn get_minecraft_versions(state: tauri::State<'_,GeneralState>) -> Result<VersionManifest, InternalError> {
    let manifest = minecraft::MinecraftAPI::get_version_manifest(&state.client).await?;
    let string = serde_json::to_string(&manifest)?;
    println!("{}", string);
    return Ok(manifest);
}