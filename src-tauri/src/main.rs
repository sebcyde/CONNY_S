// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod chatbot;
mod configuration;
mod get_dirs;
mod get_input;
mod notifications;
mod sorting;

use auth::firebase::{self, *};
use configuration::config::{get_config_data, update_config_files, UserConfig};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct UserData {
    pub user_name: String,
}

#[derive(Serialize)]
pub struct AppSettings {
    pub run_on_startup: bool,
    pub keep_watch: bool,
}

#[derive(Serialize)]
pub struct ConnyConfig {
    pub personality: String,
}

#[derive(Serialize)]
pub struct UserConfig {
    pub user_data: UserData,
    pub conny_settings: ConnyConfig,
    pub app_settings: AppSettings,
}

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub new_details: UserConfig,
}

// TODO - SWAP OUT OLD USER FOR ACTUAL USER STRUCT

//// Testing FB backend
#[tauri::command]
async fn create_new_user(new_user: User) {
    // Only works when defined here - params arent working
    create_user(new_user).await;
}

#[tauri::command]
async fn get_users() -> HashMap<String, User> {
    let users: HashMap<String, User> = get_all_users().await;
    println!("Users from backend: {:?}", &users);
    return users;
}

#[tauri::command]
async fn update_user_details(new_details: User) {
    println!("IN UPDATE FN");
    update_user(new_details).await;
    println!("POST UPDATE FN");
}

#[tauri::command]
async fn delete_existing_user(user_id: String) {
    delete_user(&user_id).await;
}

#[tauri::command]
async fn get_fb_uri() {
    get_uri().await;
}

//////////////////////////////////////////////

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
            update_user_details,
            get_user_config,
            clean_dirs,
            create_new_user,
            get_users,
            delete_existing_user,
            get_fb_uri
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
