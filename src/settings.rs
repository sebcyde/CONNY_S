use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

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

    let setup_config = move |_| {
        spawn_local(async move {
            let args: JsValue = to_value("").unwrap();
            invoke("run_setup_config", args).await;
        });
    };

    let (config, set_config) = create_signal(JsValue::NULL);

    spawn_local(async move {
        // Optional Setup
        invoke("run_setup_config", to_value("").unwrap()).await;

        // Pull Config
        let user_config: JsValue = invoke("get_user_config", to_value("").unwrap()).await;
        set_config.set(user_config);

        println!("{:?}", config);
    });

    // pub struct UserData {
    //     pub user_name: String,
    //     // pub role: String,
    // }

    // #[derive(Serialize, Deserialize)]
    // pub struct AppSettings {
    //     pub run_on_startup: bool,
    // }

    // #[derive(Serialize, Deserialize)]
    // pub struct ConnyConfig {
    //     pub personality: String, // Personality Enum
    // }

    // #[derive(Serialize, Deserialize)]
    // pub struct UserConfig {
    //     pub user_data: UserData,
    //     pub conny_settings: ConnyConfig,
    //     pub app_settings: AppSettings,
    // }

    view! {
    <div class="settings_page">
            <h2 class="PageTitle">"Settings"</h2>

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

            <button on:click=setup_config>{saving_text}</button>


            </form>
            <a href="/">"Back to Home"</a>

        </div>
    }
}
