pub mod autosorter {

    use crate::functions::directories::get_conny_directory;
    use std::path::{Path, PathBuf};

    pub async fn create_sorting_folders() {
        let user_files_dir_path: PathBuf = get_conny_directory();
        if !Path::new(&user_files_dir_path).exists() {
            _ = std::fs::create_dir_all(&user_files_dir_path);
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
                "Folders",
                "Archive",
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
    }
}
