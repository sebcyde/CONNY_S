pub mod backup {
    use std::path::{Path, PathBuf};
    use std::{
        collections::HashMap,
        process::{Command, Output},
    };
    use walkdir::WalkDir;

    use crate::{
        configuration::config::{get_config_data, update_config_files, SortSettings, UserConfig},
        functions::directories::{get_conny_directory, get_documents_dir},
    };

    fn find_file_by_name(name: &str) -> Option<String> {
        let root_docs_dir = get_documents_dir();
        for entry in WalkDir::new(root_docs_dir.to_str().unwrap()) {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("Current File: {}", entry.path().display().to_string());
                    if file_name == name {
                        return Some(entry.path().display().to_string());
                    }
                }
            }
        }
        None
    }

    fn find_directory_by_name(name: &str) -> Option<String> {
        let root_docs_dir = get_documents_dir();
        for entry in WalkDir::new(root_docs_dir.to_str().unwrap())
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("Current Dir: {}", entry.path().display().to_string());
                    if file_name == name {
                        return Some(entry.path().display().to_string());
                    }
                }
            }
        }
        None
    }

    // CAREFULE - SUPER EXPENSIVE
    async fn get_locations() -> HashMap<String, String> {
        let fc_dir: &str = "fc-cms-website";
        let es_dir: &str = "es-cms-website";
        let re_dir: &str = "re-cms-website";

        let fc_location: String = find_directory_by_name(fc_dir).unwrap();
        let es_location: String = find_directory_by_name(es_dir).unwrap();
        let re_location: String = find_directory_by_name(re_dir).unwrap();

        let mut locations: HashMap<String, String> = HashMap::new();
        locations.insert("flight_club_uk_location".to_owned(), fc_location);
        locations.insert("electric_shuffle_uk_location".to_owned(), es_location);
        locations.insert("red_engine_uk_location".to_owned(), re_location);

        return locations;
    }

    async fn backup_database(db_path: String) {
        // Change the current working directory to the root directory
        Command::new("sh")
            .arg("-c")
            .arg("cd -")
            .spawn()
            .expect("Failed to change directory to root.");

        // Change the current working directory to the database path
        let path_buf = PathBuf::from(db_path.replace("\\\\", "\\"));

        Command::new("sh")
            .arg("-c")
            .arg(format!("cd {:?} && pwd", path_buf))
            .spawn()
            .expect("Failed to list files in directory.");

        // let backups_dir: PathBuf = get_database_backups_dir();
    }

    pub async fn backup_all_databases() {
        let mut user_config: UserConfig = get_config_data().await;

        // Check if locations already exist, else find and add them
        if user_config.sort_settings.flight_club_uk_location.eq("") {
            let locations: HashMap<String, String> = get_locations().await;

            user_config.sort_settings.flight_club_uk_location =
                locations.get("flight_club_uk_location").unwrap().to_owned();

            user_config.sort_settings.electric_shuffle_uk_location = locations
                .get("electric_shuffle_uk_location")
                .unwrap()
                .to_owned();

            user_config.sort_settings.red_engine_uk_location =
                locations.get("red_engine_uk_location").unwrap().to_owned();
        }

        println!("FC: {}", &user_config.sort_settings.flight_club_uk_location);
        println!(
            "ES: {}",
            &user_config.sort_settings.electric_shuffle_uk_location
        );
        println!("RE: {}", &user_config.sort_settings.red_engine_uk_location);

        backup_database(user_config.sort_settings.flight_club_uk_location.clone()).await;
        backup_database(
            user_config
                .sort_settings
                .electric_shuffle_uk_location
                .clone(),
        )
        .await;
        backup_database(user_config.sort_settings.red_engine_uk_location.clone()).await;

        // WRITE FOUND LOCATIONS TO CONFIG - WAY TOO INSTENSIVE TO KEEP SEARCHING
        update_config_files(user_config).await;
    }
}
