pub mod autosorter {

    use crate::functions::directories::{get_conny_directory, get_downloads_dir};
    use std::path::{Path, PathBuf};

    pub async fn create_sorting_folders() {
        let user_files_dir_path: PathBuf = get_conny_directory();
        if !Path::new(&user_files_dir_path).exists() {
            _ = std::fs::create_dir_all(&user_files_dir_path);
            let supported_file_types: Vec<&str> = vec![
                "Images",
                "Videos",
                "Audio",
                "Archive",
                "Book",
                "Documents",
                "Fonts",
                "Applications",
                "Other",
                "Custom",
                "Folders",
                "Database_Backups",
            ];
            for type_str in supported_file_types {
                let mut current_dir: PathBuf = user_files_dir_path.clone();
                current_dir.push(type_str);
                _ = std::fs::create_dir_all(&current_dir);
            }
        }
    }

    pub fn sort_continuous() {}

    pub fn sort_once() {
        std::thread::sleep(std::time::Duration::from_secs(3));
        let downloads_path: PathBuf = get_downloads_dir().await;
    }
}
