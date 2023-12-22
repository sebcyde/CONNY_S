pub mod firebase {

    use firebase_rs::Firebase;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub name: String,
    }

    const APP_URL: &str = "https://conny-s-default-rtdb.europe-west1.firebasedatabase.app/";

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

    pub async fn get_user(user_id: &str) {
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users").at(user_id);

        println!("Firebase from get users: {:?}", firebase);
    }

    pub async fn update_user(user_id: &str, new_user_details: User) {
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users").at(user_id);
        let users = firebase.update::<User>(&new_user_details).await;
        println!("Users: {:?}", users);
    }

    pub async fn delete_user(user_id: &str) {
        let firebase: Firebase = Firebase::new(APP_URL).unwrap().at("users").at(user_id);
        _ = firebase.delete().await;
    }

    // fn string_to_response(s: &str) -> Response {
    //     return serde_json::from_str(s).unwrap();
    // }

    fn string_to_user(s: &str) -> User {
        return serde_json::from_str(s).unwrap();
    }

    // // Import the functions you need from the SDKs you need
    // import { initializeApp } from "firebase/app";
    // // TODO: Add SDKs for Firebase products that you want to use
    // // https://firebase.google.com/docs/web/setup#available-libraries

    // // Your web app's Firebase configuration
    // const firebaseConfig = {
    //   apiKey: "AIzaSyDeObZzfqq3uA8Mz6cKrFd1m8KAz779ijg",
    //   authDomain: "conny-s.firebaseapp.com",
    //   databaseURL: "https://conny-s-default-rtdb.europe-west1.firebasedatabase.app",
    //   projectId: "conny-s",
    //   storageBucket: "conny-s.appspot.com",
    //   messagingSenderId: "684683234929",
    //   appId: "1:684683234929:web:0baf75348f16bd82cb95d0"
    // };

    // // Initialize Firebase
    // const app = initializeApp(firebaseConfig);
}
