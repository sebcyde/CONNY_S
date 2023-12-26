pub mod updates {
    use crate::configuration::config::{get_config_data, UserConfig};
    use std::process::{Command, Output};
    use std::{path::PathBuf, process::Child};
    use tokio::io::{AsyncBufReadExt, BufReader};

    pub async fn pull(db_path: String) {
        // Change the current working directory to the database path
        let path_buf = PathBuf::from(db_path.replace("\\\\", "\\"));
        let string_path = path_buf.to_string_lossy().to_string();

        println!("Pulling from: {}", &string_path);

        let child: Child = Command::new("git")
            .args(&["-C", &string_path, "pull"])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute git pull command.");

        let output: Output = child.wait_with_output().expect("Failed to read stdout");
        println!(
            "Pull Output: {}",
            std::str::from_utf8(&output.stdout).unwrap()
        );
    }

    pub async fn pull_all() {
        let user_config: UserConfig = get_config_data().await;

        pull(user_config.sort_settings.flight_club_uk_location.clone()).await;
        pull(
            user_config
                .sort_settings
                .electric_shuffle_uk_location
                .clone(),
        )
        .await;
        pull(user_config.sort_settings.red_engine_uk_location.clone()).await;
    }
}
