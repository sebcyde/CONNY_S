use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Home() -> impl IntoView {
    let (is_cleaning, set_is_cleaning) = create_signal(false);

    let clean_directories = move |_| {
        spawn_local(async move {
            set_is_cleaning.set(true);
            let args: JsValue = to_value("").unwrap();
            invoke("clean_dirs", args).await;
            set_is_cleaning.set(false);
        });
    };

    let cleaning_text = move || match is_cleaning.get() {
        true => "Sorting...",
        false => "Sort Files",
    };

    view! {
    <div class="HomePage">
        <div class="row">
            <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
        </div>

        <p>"Welcome To Conny."</p>

        <div class="MainMenu">
            <button on:click=clean_directories>{cleaning_text}</button>
            <a href="/Settings">"Settings"</a>
            <a href="/Upcoming">"Upcoming Features"</a>
            <a href="/Other">"Testing Not Found Link"</a>
        </div>

    </div>
        }
}

#[component]
pub fn Settings() -> impl IntoView {
    let setup_config = move |_| {
        spawn_local(async move {
            let args: JsValue = to_value("").unwrap();
            invoke("run_setup_config", args).await;
        });
    };

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


            <button on:click=setup_config>Save</button>

            </form>
            <a href="/">"Back to Home"</a>

        </div>
    }
}

#[component]
pub fn Upcoming() -> impl IntoView {
    view! {
        <div class="UpcomingPage">
            <h2 class="PageTitle">"Upcoming Features"</h2>
            <a href="/">"Back to Home"</a>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="NotFoundPage">
            <h2 class="PageTitle">"Not Found"</h2>
            <a href="/">"Back to Home"</a>
        </div>
    }
}
