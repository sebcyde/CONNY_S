// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod chatbot;
mod configuration;
mod get_dirs;
mod get_input;
mod notifications;
mod sorting;

use auth::firebase::*;
use configuration::config::{get_config_data, update_config_files, UserConfig};

//// Testing FB backend
#[tauri::command]
async fn create_new_user(new_user: User) {
    // Only works when defined here - params arent working
    // let new_user: User = User {
    //     name: String::from("Sebastian"),
    // };

    create_user(new_user).await;
}

#[tauri::command]
fn get_existing_user(user_id: String) {
    println!("Inside Get Existing User");
    // get_user(&user_id).await;
}

#[tauri::command]
async fn delete_existing_user(user_id: String) {
    delete_user(&user_id).await;
}

#[tauri::command]
async fn get_fb_uri() {
    get_uri().await;
}

////////////////

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
            clean_dirs,
            //
            create_new_user,
            get_existing_user,
            delete_existing_user,
            get_fb_uri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
