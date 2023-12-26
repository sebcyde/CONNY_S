pub mod directories {
    use dirs::{config_dir, document_dir, download_dir, home_dir};
    use std::path::PathBuf;
    use std::process::Command;

    pub fn get_config_root() -> PathBuf {
        let mut config_root: PathBuf = config_dir().unwrap();
        config_root.push("Conny");
        return config_root;
    }

    pub fn get_conny_directory() -> PathBuf {
        let mut documents_dir: PathBuf = home_dir().unwrap();
        documents_dir.push("Documents");
        documents_dir.push("Conny");
        return documents_dir;
    }

    pub fn get_config_file() -> PathBuf {
        let mut config_root: PathBuf = config_dir().unwrap();
        config_root.push("Conny");
        config_root.push("config.json");
        return config_root;
    }

    pub fn get_downloads_dir() -> PathBuf {
        let downloads_dir: PathBuf = download_dir().unwrap();
        return downloads_dir;
    }

    pub fn get_database_backups_dir() -> PathBuf {
        let mut backups_dir: PathBuf = get_conny_directory();
        backups_dir.push("Database_Backups");
        return backups_dir;
    }

    

    pub fn get_documents_dir() -> PathBuf {
        let mut documents_dir: PathBuf = home_dir().unwrap();
        documents_dir.push("Documents");
        return documents_dir;
    }

    pub fn find_dir(dir_name: &str) -> Result<String, String> {
        let output = Command::new("where")
            .arg("/R") // Recursive search
            .arg("/Q") // Quiet mode (to avoid error messages)
            .arg("/F") // Display full path for the matching files
            .arg("fc-cms.local") // The directory to search
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let result: String = String::from_utf8_lossy(&output.stdout).to_string();
                    println!("Found directories:\n{}", result);
                    return Ok(result);
                } else {
                    let error: String = String::from_utf8_lossy(&output.stderr).to_string();
                    eprintln!("Error: {}", error);
                    return Err(error);
                }
            }
            Err(e) => {
                eprintln!("Error executing command: {}", e);
                return Err(e.to_string());
            }
        }
    }
}

pub mod helpers {
    use super::directories::find_dir;

    pub fn get_fc_dir() -> String {
        let dir: String = find_dir("fc-cms.local").expect("Failed to find FC dir.");
        return dir;
    }

    pub fn get_es_dir() -> String {
        let dir: String = find_dir("es-cms.local").expect("Failed to find FC dir.");
        return dir;
    }

    pub fn get_re_dir() -> String {
        let dir: String = find_dir("fc-cms.local").expect("Failed to find FC dir.");
        return dir;
    }
}
