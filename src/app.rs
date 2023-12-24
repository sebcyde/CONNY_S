use crate::helpers::{self, functions::UserConfig};
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Home() -> impl IntoView {
    let (welcome_text, set_welcome_text) = create_signal::<String>(String::new());

    spawn_local(async move {
        let config: UserConfig = helpers::functions::get_user_details().await;
        set_welcome_text.set(format!("Welcome Back {}", config.user_data.user_name));
    });

    view! {
        <div class="HomePage">
            <div class="row">
                <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
            </div>

            <p>{move || welcome_text.get()}</p>

            <div class="MainMenu">
                <a href="/Chat">"Chat"</a>
                <a href="/Sorting">"File Sorting"</a>
                <a href="/Settings">"Settings"</a>
                <a href="/Upcoming">"Upcoming Features"</a>
                <a href="/Other">"Testing Not Found Link"</a>
            </div>
        </div>
    }
}
