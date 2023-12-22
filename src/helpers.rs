pub mod functions {

    use std::collections::HashMap;

    use leptos::*;
    use serde::{Deserialize, Serialize};
    use serde_json::to_string_pretty;
    use serde_wasm_bindgen::to_value;
    use tauri_sys::tauri;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
        async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    }

    pub async fn get_users_data() -> String {
        let data: JsValue = invoke("get_users", to_value("").unwrap()).await;
        let json_value: serde_json::Value = serde_wasm_bindgen::from_value(data).unwrap();
        let json_string: String = to_string_pretty(&json_value).unwrap();
        json_string
    }

    ///////////////////////////////////////////////////////////////
    #[derive(Serialize)]
    pub struct UserData {
        pub user_name: String,
    }

    #[derive(Serialize)]
    pub struct AppSettings {
        pub run_on_startup: bool,
        pub keep_watch: bool,
    }

    #[derive(Serialize)]
    pub struct ConnyConfig {
        pub personality: String,
    }

    #[derive(Serialize)]
    pub struct UserConfig {
        pub user_data: UserData,
        pub conny_settings: ConnyConfig,
        pub app_settings: AppSettings,
    }

    #[derive(Serialize)]
    pub struct UpdateParams {
        pub id: String,
        pub new_details: UserConfig,
    }

    pub async fn update_user(update_details: UpdateParams) {
        let args: JsValue = to_value(&update_details).unwrap();
        println!("Args: {:?}", &args);

        // let mut params = HashMap::new();
        // params.insert("id", serde_wasm_bindgen::to_value(&id));
        // params.insert("new_details", serde_wasm_bindgen::to_value(&new_details));
        // let args: JsValue = serde_wasm_bindgen::to_value(&params).unwrap();

        let _data: JsValue = invoke("update_user_details", args).await;
    }
}
