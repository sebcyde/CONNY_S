use crate::helpers::{self, functions::UserConfig};
use leptos::{leptos_dom::logging::console_log, *};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Home() -> impl IntoView {
    let (welcome_text, set_welcome_text) = create_signal::<String>(String::new());

    let (constant_sort, set_constant_sort) = create_signal::<bool>(false);

    spawn_local(async move {
        let config: UserConfig = helpers::functions::get_user_details().await;
        set_welcome_text.set(format!("Welcome Back {}", config.user_data.user_name));
        set_constant_sort.set(match config.app_settings.constant_watch {
            true => true,
            false => false,
        })
    });

    // Sorting Functionality
    let (is_sorting, set_is_sorting) = create_signal::<bool>(false);

    let sort_files = move |_| {
        spawn_local(async move {
            set_is_sorting.set(true);
            invoke("clean_dirs", to_value("").unwrap()).await;
            set_is_sorting.set(false);
        })
    };

    let sorting_text = move || match constant_sort.get() {
        true => "Persistent Sort Active",
        false => match is_sorting.get() {
            true => "Sorting...",
            false => "Sort Local Files",
        },
    };

    view! {
        <div class="HomePage">

            // <p>{move || welcome_text.get()}</p>

            <div class="MainMenu">
                <a href="/Chat">"Chat"</a>
                <button disabled=move || constant_sort.get() on:click=sort_files>{sorting_text}</button>
                <a href="/Backups">"Backup Databases"</a>
                <a href="/Updates">"Pull Repositories"</a>
                <a href="/Settings">"Settings"</a>
                // <a href="/Upcoming">"Upcoming Features"</a>
                // <a href="/Other">"Testing Not Found Link"</a>
            </div>
        </div>
    }
}
