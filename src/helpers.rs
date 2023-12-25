pub mod functions {

    use leptos::{leptos_dom::logging::console_log, *};
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen::from_value;
    use serde_wasm_bindgen::to_value;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
        async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct UserData {
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AppSettings {
        pub run_on_startup: bool,
        pub constant_watch: bool,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ConnyConfig {
        pub personality: String,
    }
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct UserConfig {
        pub user_data: UserData,
        pub conny_settings: ConnyConfig,
        pub app_settings: AppSettings,
    }

    /////////////// General Functions

    pub fn convert_js_value_to_string(data: JsValue) -> String {
        return data.as_string().unwrap();
    }

    pub fn get_default_user_config() -> UserConfig {
        let user_data: UserData = UserData {
            user_name: String::from("Default_User"),
        };
        let conny_settings: ConnyConfig = ConnyConfig {
            personality: String::from("Standard"),
        };
        let app_settings: AppSettings = AppSettings {
            run_on_startup: false,
            constant_watch: false,
        };

        return UserConfig {
            user_data,
            conny_settings,
            app_settings,
        };
    }

    pub fn _create_user_config(
        user_name: String,
        personality: String,
        run_on_startup: bool,
        constant_watch: bool,
    ) -> UserConfig {
        let user_data = UserData { user_name };
        let conny_settings = ConnyConfig { personality };
        let app_settings = AppSettings {
            run_on_startup,
            constant_watch,
        };

        return UserConfig {
            user_data,
            conny_settings,
            app_settings,
        };
    }

    /////////////// User Functions

    pub async fn get_user_details() -> UserConfig {
        let data: JsValue = invoke("get_user", to_value("").unwrap()).await;
        return from_value(data).unwrap();
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct DataArgs<'a> {
        newDetails: &'a str,
    }

    /////////////// PARAMETER V? Functions
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ParamUserData {
        pub user_name: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ParamAppSettings {
        pub run_on_startup: bool,
        pub constant_watch: bool,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ParamConnyConfig {
        pub personality: String,
    }
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ParamUserConfig {
        // NEEDS TO BE CAMELCASE
        pub userData: UserData,
        pub connySettings: ConnyConfig,
        pub appSettings: AppSettings,
    }

    pub async fn update_user_details(update_details: UserConfig) {
        invoke(
            "update_user",
            to_value(&ParamUserConfig {
                userData: update_details.user_data,
                appSettings: update_details.app_settings,
                connySettings: update_details.conny_settings,
            })
            .unwrap(),
        )
        .await;
    }

    pub async fn reset_user_details() {
        invoke("reset_user", to_value("").unwrap()).await;
    }
}
