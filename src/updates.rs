use crate::helpers::{self, functions::UserConfig};
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RepoParam {
    path: String,
}

#[component]
pub fn Updates() -> impl IntoView {
    let (is_All, set_is_All) = create_signal(false);
    let (is_FC, set_is_FC) = create_signal(false);
    let (is_ES, set_is_ES) = create_signal(false);
    let (is_RE, set_is_RE) = create_signal(false);

    let FC_text = move || match is_FC.get() {
        true => "Pulling Flight Club...",
        false => "Flight Club",
    };

    let ES_text = move || match is_ES.get() {
        true => "Pulling Electric Shuffle...",
        false => "Electric Shuffle",
    };

    let RE_text = move || match is_RE.get() {
        true => "Pulling Red Engine...",
        false => "Red Engine",
    };

    // let all_text = move || match is_All.get() {
    //     true => "Pulling All Repos...",
    //     false => "Pull All Repos",
    // };

    // let update_all_repos = move |_| {
    //     spawn_local(async move {
    //         set_is_All.set(true);
    //         invoke("pull_all_repos", to_value("").unwrap()).await;
    //         set_is_All.set(false);
    //     })
    // };

    let update_fc = move |_| {
        spawn_local(async move {
            set_is_FC.set(true);
            let config: UserConfig = helpers::functions::get_user_details().await;
            invoke(
                "pull_single_repo",
                to_value(&RepoParam {
                    path: config.sort_settings.flight_club_uk_location,
                })
                .unwrap(),
            )
            .await;
            set_is_FC.set(false);
        })
    };

    let update_es = move |_| {
        spawn_local(async move {
            set_is_ES.set(true);
            let config: UserConfig = helpers::functions::get_user_details().await;
            invoke(
                "pull_single_repo",
                to_value(&RepoParam {
                    path: config.sort_settings.electric_shuffle_uk_location,
                })
                .unwrap(),
            )
            .await;
            set_is_ES.set(false);
        })
    };

    let update_re = move |_| {
        spawn_local(async move {
            set_is_RE.set(true);
            let config: UserConfig = helpers::functions::get_user_details().await;
            invoke(
                "pull_single_repo",
                to_value(&RepoParam {
                    path: config.sort_settings.red_engine_uk_location,
                })
                .unwrap(),
            )
            .await;
            set_is_RE.set(false);
        })
    };

    view! {
        <div class="UpdatesPage">
            <h2 class="page_title">"Pull Newest Codebase"</h2>

                <button class="Themed-Button" on:click=update_fc>{FC_text}</button>

                <button class="Themed-Button" on:click=update_es>{ES_text}</button>

                <button class="Themed-Button" on:click=update_re>{RE_text}</button>

                // <button class="Themed-Button" on:click=update_all_repos>{all_text}</button>

                <a class="Themed-Button" href="/">"Back to Home"</a>

            </div>
    }
}
