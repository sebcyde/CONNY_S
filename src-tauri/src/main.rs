// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod auth;
mod chatbot;
mod configuration;
mod get_dirs;
mod get_input;
mod monitoring;
mod notifications;
mod sorting;

use configuration::config::{get_config_data, reset_config_files, update_config_files, UserConfig};

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
async fn update_user(new_details: UserConfig) {
    println!("IN UPDATE FN");
    update_config_files(new_details).await;
    println!("POST UPDATE FN");
}

#[tauri::command]
async fn reset_user() {
    println!("IN RESET FN");
    reset_config_files().await;
    println!("POST RESET FN");
}

#[tauri::command]
async fn get_user() -> UserConfig {
    return get_config_data().await;
}

////////////////// System Commands

#[tauri::command]
async fn clean_dirs() {
    println!("CLEANING DIRS");
    // TODO - ADD IN CLEANING
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Cleaning Complete.");
}

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
