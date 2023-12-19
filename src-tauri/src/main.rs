// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn clean_dirs() -> i32 {
    println!("CLEANING DIRS");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Cleaning Complete.");
    return 0;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, clean_dirs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
