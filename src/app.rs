use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserArgs {
    user_id: String,
}

#[component]
pub fn Home() -> impl IntoView {
    let (is_cleaning, set_is_cleaning) = create_signal(false);
    let (config, set_config) = create_signal(JsValue::NULL);

    // let clean_directories = move |_| {
    //     spawn_local(async move {
    //         set_is_cleaning.set(true);
    //         let args: JsValue = to_value("").unwrap();
    //         invoke("clean_dirs", args).await;
    //         set_is_cleaning.set(false);
    //     });
    // };

    let cleaning_text = move || match is_cleaning.get() {
        true => "testing...",
        false => "Sort Files",
    };

    /////////////////////////////////
    ///
    /// - PARAMS ARENT WORKING!!!!
    ///
    /////////////////////////////////

    // spawn_local(async move {
    //     let user_config: JsValue = invoke("get_user_config", to_value("").unwrap()).await;
    //     set_config.set(user_config);
    // });
    let create_user = move |_| {
        set_is_cleaning.set(true);
        spawn_local(async move {
            // let new_user: User = User {
            //     name: String::from("Sebastian"),
            // };

            let args: JsValue = to_value("").unwrap();
            println!("Args: {:?}", &args);

            invoke("create_new_user", args).await;
        });

        set_is_cleaning.set(false);
    };

    let get_user = move |_| {
        spawn_local(async move {
            let args: JsValue = to_value(&GetUserArgs {
                user_id: String::from("NmElTd5XLZfwDZF7WPh"),
            })
            .unwrap();

            invoke("get_existing_user", args).await;
        });
    };

    let delete_user = move |_| {
        spawn_local(async move {
            let args: JsValue = to_value(&"NmElTd5XLZfwDZF7WPh").unwrap();
            invoke("delete_existing_user", args).await;
        });
    };

    let get_uri = move |_| {
        spawn_local(async move {
            let args: JsValue = to_value("").unwrap();
            invoke("get_fb_uri", args).await;
        });
    };

    view! {
    <div class="HomePage">
        <div class="row">
            <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
        </div>

        <p>"Welcome To Conny."</p>

        <div class="MainMenu">
            // <button on:click=clean_directories>{cleaning_text}</button>
            // <a href="/Chat">"Chat"</a>
            // <a href="/Settings">"Settings"</a>
            // <a href="/Upcoming">"Upcoming Features"</a>
            // <a href="/Other">"Testing Not Found Link"</a>

            // Testing backend
            <button on:click=create_user>{cleaning_text}</button>
            <button on:click=get_user>"get_existing_user"</button>
            <button on:click=delete_user>"delete_existing_user"</button>
            <button on:click=get_uri>"get_uri"</button>
            //
            </div>

    </div>
        }
}
