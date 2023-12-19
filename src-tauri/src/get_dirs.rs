pub mod get_dirs {
    use dirs::{config_dir, document_dir, download_dir, home_dir};
    use std::path::PathBuf;

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

    pub fn get_documents_dir() -> PathBuf {
        let mut documents_dir: PathBuf = home_dir().unwrap();
        documents_dir.push("Documents");
        return documents_dir;
    }
}
