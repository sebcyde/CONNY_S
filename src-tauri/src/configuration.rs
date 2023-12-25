pub mod config {

    use serde::{Deserialize, Serialize};

    use crate::functions::directories::{get_config_file, get_config_root};
    use std::fs::read_to_string;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserData {
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AppSettings {
        pub run_on_startup: bool,
        pub constant_watch: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ConnyConfig {
        pub personality: String, // Personality Enum
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserConfig {
        pub user_data: UserData,
        pub conny_settings: ConnyConfig,
        pub app_settings: AppSettings,
    }

    pub fn get_default_user_config() -> UserConfig {
        let user_data: UserData = UserData {
            user_name: String::from("R_Default_User"),
        };
        let conny_settings: ConnyConfig = ConnyConfig {
            personality: String::from("Standard"),
        };
        let app_settings: AppSettings = AppSettings {
            run_on_startup: false,
            constant_watch: false,
        };

        return UserConfig {
            user_data,
            conny_settings,
            app_settings,
        };
    }

    async fn set_default_config_files() {
        std::fs::create_dir_all(&get_config_root()).unwrap();
        let config_path: PathBuf = get_config_file();
        let mut config_file: File = File::create(&config_path).unwrap();
        let json_data: String = serde_json::to_string(&get_default_user_config()).unwrap();
        _ = config_file.write_all(json_data.as_bytes());
    }

    pub async fn get_config_data() -> UserConfig {
        let config_path: PathBuf = get_config_file();
        if !std::path::Path::exists(&config_path) {
            set_default_config_files().await;
        }
        let config_value: &str = &read_to_string(config_path).unwrap();
        let config: UserConfig = serde_json::from_str(config_value).unwrap();
        return config;
    }

    pub async fn update_config_files(user_config: UserConfig) {
        let user_config_dir_path: PathBuf = get_config_root();
        if !Path::new(&user_config_dir_path).exists() {
            set_default_config_files().await;
        }
        let config_path: PathBuf = get_config_file();
        let mut config_file: File = File::create(&config_path).unwrap();
        let json_data: String = serde_json::to_string(&user_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());
    }

    pub async fn reset_config_files() {
        let user_config_dir_path: PathBuf = get_config_root();
        std::fs::remove_dir_all(&user_config_dir_path).unwrap();
        set_default_config_files().await;
    }
}
