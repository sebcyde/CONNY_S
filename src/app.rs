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
    let (config, set_config) = create_signal(JsValue::NULL);

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

    spawn_local(async move {
        let user_config: JsValue = invoke("get_user_config", to_value("").unwrap()).await;
        set_config.set(user_config);
    });

    view! {
    <div class="HomePage">
        <div class="row">
            <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
        </div>

        <p>"Welcome To Conny."</p>

        <div class="MainMenu">
            <button on:click=clean_directories>{cleaning_text}</button>
            <a href="/Chat">"Chat"</a>
            <a href="/Settings">"Settings"</a>
            <a href="/Upcoming">"Upcoming Features"</a>
            <a href="/Other">"Testing Not Found Link"</a>
        </div>

    </div>
        }
}
