use crate::helpers::{self, functions::UserConfig};
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
    let (user_data, set_user_data) = create_signal::<String>(String::new());

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

    let get_user_config = move || {
        spawn_local(async move {
            let config: UserConfig = helpers::functions::get_user_details().await;
            set_user_data.set(config.ser().unwrap());
        });
    };

    view! {
    <div class="HomePage">
        <div class="row">
            <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
        </div>

        <p>"Welcome Back."</p>

        <div class="MainMenu">
            <button on:click=clean_directories>{cleaning_text}</button>
            <a href="/Chat">"Chat"</a>
            <a href="/Settings">"Settings"</a>
            <a href="/Upcoming">"Upcoming Features"</a>
            <a href="/Other">"Testing Not Found Link"</a>

            // Testing backend
            // <button on:click=create_user>{cleaning_text}</button>
            // <button on:click=delete_user>"delete_existing_user"</button>
            // <button on:click=get_uri>"get_uri"</button>
            </div>

    </div>
        }
}
