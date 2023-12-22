use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::helpers::{
    self,
    functions::{AppSettings, ConnyConfig, UpdateParams, UserConfig, UserData},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Settings() -> impl IntoView {
    let (is_saving, set_is_saving) = create_signal(false);

    let saving_text = move || match is_saving.get() {
        true => "Saving...",
        false => "Save",
    };

    let (all_user_data, set_all_user_data) = create_signal(String::new());

    let (config, set_config) = create_signal(JsValue::NULL);

    let get_users = move |_| {
        spawn_local(async move {
            let json_string: String = helpers::functions::get_users_data().await;
            set_all_user_data.set(json_string.as_str().to_owned());
        });
    };

    let update_user = move |_| {
        spawn_local(async move {
            let user_data = UserData {
                user_name: "-NmElTd5XLZfwDZF7WPh".to_string(), // Placeholder user name
            };

            let app_settings = AppSettings {
                run_on_startup: true, // Placeholder values for AppSettings
                keep_watch: false,
            };

            let conny_config = ConnyConfig {
                personality: "Friendly".to_string(), // Placeholder personality
            };

            let user_config = UserConfig {
                user_data,
                conny_settings: conny_config,
                app_settings,
            };

            let update_params = UpdateParams {
                id: "user_id_here".to_string(), // Placeholder user ID
                new_details: user_config,
            };

            helpers::functions::update_user(update_params).await;
            // set_all_user_data.set(json_string.as_str().to_owned());
        });
    };

    view! {
    <div class="settings_page">
            <h2 class="PageTitle">"Settings"</h2>

            <p>{all_user_data}</p>

            <form>
            // User Settings
            <input placeholder="User Name"/>
            <input placeholder="User Role"/>

            // App Settings
            <input placeholder="Run on startup?"/>
            <input placeholder="File by date?"/>


            // Conny Settings
            <input placeholder="Personality type"/>
            <input placeholder="User Name"/>

            <button on:click=update_user>{saving_text}</button>
            <button on:click=get_users>"get_existing_user"</button>

            </form>
            <a href="/">"Back to Home"</a>

        </div>
    }
}
