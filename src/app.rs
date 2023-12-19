use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::{thread, time};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FunctionResponse {
    response: i32,
}

#[component]
pub fn Home() -> impl IntoView {
    let (is_cleaning, set_is_cleaning) = create_signal(false);

    let Nav = move |Route: &str| {
        println!(" Navigating to route: {}", Route);
        // tauri::WindowUrl::App("result.html".into())
    };

    let clean_directories = move |_| {
        spawn_local(async move {
            set_is_cleaning.set(true);
            // thread::sleep(time::Duration::from_secs(3));
            let args: JsValue = to_value("").unwrap();
            invoke("clean_dirs", args).await;
            set_is_cleaning.set(false);
        });
    };

    let cleaning_text = move || match is_cleaning.get() {
        true => "Cleaning...",
        false => "Clean Files",
    };

    view! {
        <div>
            <div class="row">
                <a href="" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>

            </div>

            <p>"Welcome To Conny."</p>

            <div class="MainMenu">

                <button on:click=clean_directories>{cleaning_text}</button>

                <button>"Settings"</button>
                <button>"Upcoming Features"</button>
            </div>
        </div>

    }
}

#[component]
pub fn Settings() -> impl IntoView {
    let Nav = move |Route: &str| {
        println!(" Navigating to route: {}", Route);
    };

    view! {
        <main class="container">
            <p>"Settings"</p>
        </main>
    }
}

#[component]
pub fn Upcoming() -> impl IntoView {
    let Nav = move |Route: &str| {
        println!(" Navigating to route: {}", Route);
    };

    view! {
        <main class="container">
            <p>"Upcoming Features"</p>
        </main>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    let Nav = move |Route: &str| {
        println!(" Navigating to route: {}", Route);
    };

    view! {
        <main class="container">
            <p>"Not Found"</p>
        </main>
    }
}
