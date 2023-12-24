use leptos::*;
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
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
