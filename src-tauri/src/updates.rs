pub mod updates {
    use crate::notifications::notifications::send_notif;

    pub async fn pull_database(chosen_database: &str) {
        match chosen_database {
            "flightclub" => println!("Pulling FC"),
            "electricshuffle" => println!("Pulling ES"),
            "redengine" => println!("Pulling RE"),
            "fccareers" => println!("Pulling FC Careers"),
            "escareers" => println!("Pulling ES Careers"),
            _ => println!("Incorrect parameter provided."),
        };

        let notif_content: String = format!("Pulling {}", chosen_database);
        send_notif(&notif_content);
    }

    async fn git_pull() {
        // unnecesary abstraction?
        todo!();
    }
}
