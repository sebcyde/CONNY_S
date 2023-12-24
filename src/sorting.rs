use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn NotFound() -> impl IntoView {
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
        <div class="SortingPage">
        <h2 class="PageTitle">"Not Found"</h2>
        <button on:click=clean_directories>{cleaning_text}</button>

            <a href="/">"Back to Home"</a>
        </div>
    }
}
