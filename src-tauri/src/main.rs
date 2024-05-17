// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Context;
use nl_account_manager::{Account, AccountManager};
use nl_core::java::JavaManager;
use tauri::Manager;
use tracing::{debug, error};
pub mod instance;
pub mod utils;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    human_panic::setup_panic!();

    tauri::Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.handle().path().app_data_dir()?;
    let logging_dir = app.handle().path().app_log_dir()?;
    if !logging_dir.exists() {
        std::fs::create_dir_all(&logging_dir)?;
    }
    utils::init_logging(logging_dir)?;
    {
        let accounts = AccountManager::load_from_directory(data_dir.clone())
            .context("Could not Load Account Manager")?;
        check_accounts(accounts.clone());
        app.manage(accounts);
    }
    {
        let java_manager =
            JavaManager::new(data_dir.clone()).context("Could not load java file")?;
        app.manage(java_manager);
    }

    Ok(())
}
fn check_accounts(account_manager: AccountManager) {
    tauri::async_runtime::spawn(async move {
        let mut accounts = account_manager.accounts.lock().clone();
        let mut update = false;
        for account in &mut accounts.accounts {
            let Account::Microsoft(account) = account else {
                debug!("Not checking account {:?}", account);
                continue;
            };

            match account_manager
                .account_api
                .load_account(&mut account.account_save)
                .await
            {
                Ok(ok) => update = ok || update,
                Err(err) => {
                    error!(?err, ?account, "Could not update microsoft account");
                }
            };
        }
        if update {
            {
                let mut locked = account_manager.accounts.lock();
                *locked = accounts
            }
            if let Err(err) = account_manager.save() {
                error!(?err, "Could not save Accounts Manager")
            }
        }
    });
}
