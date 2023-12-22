pub mod firebase {

    use firebase_rs::Firebase;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use crate::UpdateParams;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub name: String,
    }

    pub const APP_URL: &str = "https://conny-s-default-rtdb.europe-west1.firebasedatabase.app/";

    pub async fn get_uri() {
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users");
        let uri: String = firebase.get_uri();
        println!("URI: {}", uri);
    }

    pub async fn create_user(new_user: User) {
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users");
        let users = firebase.set::<User>(&new_user).await;
        println!("users from create users: {:?}", users);
    }

    pub async fn get_all_users() -> HashMap<String, User> {
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users");
        let users: HashMap<String, User> = firebase.get::<HashMap<String, User>>().await.unwrap();
        return users;
    }

    // pub async fn get_user(id: &str) -> User {
    //     let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users").at(id);
    //     let user: HashMap<String, User> = firebase.get::<HashMap<String, User>>().await.unwrap();
    //     return user;
    // }

    pub async fn update_user(new_user_details: User) {
        let firebase: Firebase = Firebase::new(APP_URL)
            .unwrap()
            .at("users")
            .at(&new_user_details.id);
        let users = firebase.update::<User>(&new_user_details).await;

        println!("Users: {:?}", users);
    }

    pub async fn delete_user(user_id: &str) {
        println!("Deleting user: {}", &user_id);
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users").at(user_id);
        _ = firebase.delete().await;
        println!("User deleted.");
    }

    // fn string_to_response(s: &str) -> Response {
    //     return serde_json::from_str(s).unwrap();
    // }

    fn string_to_user(s: &str) -> User {
        return serde_json::from_str(s).unwrap();
    }
}
