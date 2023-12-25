// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod auth;
mod chatbot;
mod configuration;
mod functions;
mod monitoring;
mod notifications;
mod sorting;
mod updates;

use configuration::config::{
    get_config_data, reset_config_files, update_config_files, AppSettings, ConnyConfig, UserConfig,
    UserData,
};
use serde_json::{from_value, Value};

use crate::notifications::notifications::send_notif;
use crate::sorting::autosorter::sort_once;

////// Cloud Commands
// TODO - SWAP OUT OLD USER FOR ACTUAL USER STRUCT
//
// #[tauri::command]
// async fn create_new_user(new_user: User) {
//     // Only works when defined here - params arent working
//     create_user(new_user).await;
// }

// #[tauri::command]
// async fn get_users() -> HashMap<String, User> {
//     let users: HashMap<String, User> = get_all_users().await;
//     println!("Users from backend: {:?}", &users);
//     return users;
// }

// #[tauri::command]
// async fn delete_existing_user(user_id: String) {
//     delete_user(&user_id).await;
// }

// #[tauri::command]
// async fn get_fb_uri() {
//     get_uri().await;
// }

////////////////// User Commands

#[tauri::command]
async fn update_user(userData: UserData, appSettings: AppSettings, connySettings: ConnyConfig) {
    println!("Received UserConfig in update_user function");
    let user_config: UserConfig = UserConfig {
        app_settings: appSettings,
        conny_settings: connySettings,
        user_data: userData,
    };
    update_config_files(user_config).await;
    send_notif("User setting update succesful.");
}

#[tauri::command]
async fn reset_user() {
    reset_config_files().await;
    send_notif("User setting reset succesful.");
}

#[tauri::command]
async fn get_user() -> UserConfig {
    println!("Fetching User Data");
    return get_config_data().await;
}

////////////////// System Commands

#[tauri::command]
async fn clean_dirs() {
    sort_once();
    send_notif("Sorting Complete");
}

////////////////// Main

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            update_user,
            get_user,
            reset_user,
            clean_dirs,
        ])
        .run(tauri::generate_context!())
        .expect("Error while loading Conny...");
}
