use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::helpers::{self, functions::UserConfig};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Backups() -> impl IntoView {
    let (is_All, set_is_All) = create_signal(false);
    let (is_FC, set_is_FC) = create_signal(false);
    let (is_ES, set_is_ES) = create_signal(false);
    let (is_RE, set_is_RE) = create_signal(false);

    let FC_text = move || match is_FC.get() {
        true => "Backing Up...",
        false => "Flight Club",
    };

    let ES_text = move || match is_ES.get() {
        true => "Backing Up...",
        false => "Electric Shuffle",
    };

    let RE_text = move || match is_RE.get() {
        true => "Backing Up...",
        false => "Red Engine",
    };

    let all_text = move || match is_All.get() {
        true => "Backing Up...",
        false => "Backup All",
    };

    let BFC = move |_| {
        spawn_local(async move {
            set_is_FC.set(true);
            // let config: UserConfig = helpers::functions::get_user_details().await;
            // pull(config.sort_settings.flight_club_uk_location).await;
            // helpers::functions::reset_user_details().await;
            set_is_FC.set(false);
        })
    };

    let BES = move |_| {
        spawn_local(async move {
            set_is_ES.set(true);
            // helpers::functions::reset_user_details().await;
            set_is_ES.set(false);
        })
    };

    let BRE = move |_| {
        spawn_local(async move {
            set_is_RE.set(true);
            // helpers::functions::reset_user_details().await;
            set_is_RE.set(false);
        })
    };

    let backup_all = move |_| {
        spawn_local(async move {
            set_is_All.set(true);
            invoke("backup_all", to_value("").unwrap()).await;
            set_is_All.set(false);
        })
    };

    view! {
        <div class="BackupsPage">
            // <h2 class="PageTitle">"Backups Page"</h2>

            <button on:click=BFC>{FC_text}</button>
            <button on:click=BES>{ES_text}</button>
            <button on:click=BRE>{RE_text}</button>
            <button on:click=backup_all>{all_text}</button>

            <a href="/">"Back to Home"</a>
        </div>
    }
}
