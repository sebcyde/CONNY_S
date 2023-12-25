pub mod backup {
    use std::process::{Command, Output};
    use walkdir::WalkDir;

    use crate::functions::directories::get_documents_dir;

    fn find_file_by_name(name: &str) -> Option<String> {
        let root_docs_dir = get_documents_dir();
        for entry in WalkDir::new(root_docs_dir.to_str().unwrap()) {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("Current File: {}", file_name);
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

    fn get_locations() {
        let fc_dir: &str = "fc-cms-website";
        let es_dir: &str = "es-cms-website";
        let re_dir: &str = "re-cms-website";

        let fc_location: String = find_directory_by_name(fc_dir).unwrap();
        let es_location: String = find_directory_by_name(es_dir).unwrap();
        let re_location: String = find_directory_by_name(re_dir).unwrap();

        // WRITE FOUND LOCATIONS TO CONFIG - WAY TOO INSTENSIVE SEARCH
    }

    pub async fn backup_all_databases() {}
}
