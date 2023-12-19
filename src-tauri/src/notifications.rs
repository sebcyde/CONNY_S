pub mod notifications {
    use notify_rust::{Notification, Timeout};

    pub fn send_notif(content: &str) {
        println!("{}", &content);

        Notification::new()
            .auto_icon()
            .appname("Conny-S")
            .body(content)
            .timeout(Timeout::Milliseconds(5000))
            .show()
            .unwrap();
    }
}
