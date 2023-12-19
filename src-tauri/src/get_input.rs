pub mod input {

    pub async fn send_output(content: &str) {
        println!("\n{}\n", content)
    }

    pub async fn get_input() -> Result<String, std::io::Error> {
        let mut input: String = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => Ok(input),
            Err(error) => {
                println!("Error: {:?}", error);
                Err(error)
            }
        }
    }
}
