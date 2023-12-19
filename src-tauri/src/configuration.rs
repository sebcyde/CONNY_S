pub mod config {

    use serde::{Deserialize, Serialize};

    use crate::get_dirs::get_dirs::{
        get_config_file, get_config_root, get_conny_directory, get_documents_dir,
    };
    use crate::get_input::input::{get_input, send_output};
    use std::fs::read_to_string;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    #[derive(Serialize, Deserialize)]
    pub struct UserData {
        pub user_name: String,
        // pub role: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AppSettings {
        pub run_on_startup: bool,
        pub keep_watch: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ConnyConfig {
        pub personality: String, // Personality Enum
    }

    #[derive(Serialize, Deserialize)]
    pub struct UserConfig {
        pub user_data: UserData,
        pub conny_settings: ConnyConfig,
        pub app_settings: AppSettings,
    }

    pub async fn setup_config() {
        println!("Retrieving config...");
        let conny_dir_path: PathBuf = get_conny_directory();
        let config_dir_path: PathBuf = get_config_root();

        if !Path::new(&config_dir_path).exists() {
            println!("No config file detected. Creating...");
            set_config_files().await
        }

        if !Path::new(&conny_dir_path).exists() {
            println!("No Conny directory detected. Creating...\n");
            _ = std::fs::create_dir_all(&conny_dir_path);

            let supported_file_types: Vec<&str> = vec![
                "Image",
                "Video",
                "Audio",
                "Archive",
                "Book",
                "Documents",
                "Font",
                "Application",
                "Other",
                "Custom",
            ];

            for type_str in supported_file_types {
                let mut c: PathBuf = conny_dir_path.clone();
                c.push(type_str);
                _ = std::fs::create_dir_all(&c);
            }
        }
    }

    pub async fn set_config_files() {
        println!("Starting initial setup.");
        send_output("Welcome to Clarity! What's your name?").await;
        let user_name = get_input().await;
        create_config_file(&user_name.unwrap()).await;
    }

    pub async fn get_config_data() -> UserConfig {
        let config_path: PathBuf = get_config_file();
        println!("config_path: {:?}", config_path);
        let config_value: &str = &read_to_string(config_path).unwrap();
        let config: UserConfig = serde_json::from_str(config_value).unwrap();
        return config;
    }

    async fn create_config_file(name: &str) {
        let config_path: PathBuf = get_config_root();
        std::fs::create_dir_all(&config_path).unwrap();

        let config_path: PathBuf = get_config_file();
        let mut config_file: File = File::create(&config_path).unwrap();

        let user_config: UserConfig = UserConfig {
            user_data: UserData {
                user_name: name.trim_end_matches(&['\r', '\n'][..]).to_owned(),
            },
            // watch_settings: WatchConfig {
            //     watch_documents: false,
            //     watch_downloads: true,
            //     sort_by_date: false,
            //     notifications: true,
            // },
            app_settings: AppSettings {
                // show_app_logo: true,
                run_on_startup: true,

                // TODO - Add in optional continuous watch - could get annoying
                keep_watch: false,
            },
            conny_settings: ConnyConfig {
                personality: String::from("Standard"),
            },
        };

        let json_data: String = serde_json::to_string(&user_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());
    }
}
