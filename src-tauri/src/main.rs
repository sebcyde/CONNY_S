// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod chatbot;
mod configuration;
mod get_dirs;
mod get_input;
mod notifications;
mod sorting;

use configuration::config::{get_config_data, update_config_files, UserConfig};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn clean_dirs() {
    println!("CLEANING DIRS");
    // TODO - ADD IN CLEANING
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Cleaning Complete.");
}

#[tauri::command]
async fn get_user_config() -> UserConfig {
    return get_config_data().await;
}

#[tauri::command]
async fn update_user_config(new_config: UserConfig) {
    update_config_files(new_config).await;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            update_user_config,
            get_user_config,
            clean_dirs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
