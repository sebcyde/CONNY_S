use leptos::*;
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::helpers::{self, functions::UserConfig};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Loading() -> impl IntoView {
    spawn_local(async move {
        // let config: UserConfig = helpers::functions::get_user_details().await;
        let navigate = leptos_router::use_navigate();
        navigate("/App", Default::default());
    });

    view! {
        <div class="LoadingPage">
          <img src="/public/Conny-Loading.gif"/>
        </div>
    }
}
