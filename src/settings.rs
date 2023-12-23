use leptos::*;
use wasm_bindgen::prelude::*;

use crate::helpers::{
    self,
    functions::{get_default_user_config, UserConfig},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Settings() -> impl IntoView {
    let (is_saving, set_is_saving) = create_signal(false);
    let (user_data, set_user_data) = create_signal(get_default_user_config());

    spawn_local(async move {
        let config: UserConfig = helpers::functions::get_user_details().await;
        set_user_data.set(config);
    });

    let saving_text = move || match is_saving.get() {
        true => "Saving...",
        false => "Save",
    };

    let reset_user = move |_| {
        spawn_local(async move {
            helpers::functions::reset_user_details().await;
        })
    };

    let update_user = move |_| {
        spawn_local(async move {
            // let new_config: UserConfig = create_user_config();
            // helpers::functions::update_user_details(new_config).await;
            let config: UserConfig = helpers::functions::get_user_details().await;
            set_user_data.set(config);
        });
    };

    view! {
      <div class="settings_page">
        <form>
          <div>
            <h2>"User Settings"</h2>
            <span class="FormRow">
              <p>"Current User:"</p>
              <input placeholder={move || user_data.get().user_data.user_name.to_string()}/>
            </span>
          </div>

          <div>
            <h2>"App Settings"</h2>
            <span class="FormRow">
              <p>"Run On Startup:"</p>
              <input type="checkbox" checked={move || user_data.get().app_settings.run_on_startup}/>
            </span>

            <span class="FormRow">
              <p>"Sort Files By Date:"</p>
              <input type="checkbox" checked={move || user_data.get().app_settings.run_on_startup.to_string()}/>
            </span>

            <span class="FormRow">
              <p>"Persistent Sort:"</p>
              <input type="checkbox" checked={move || user_data.get().app_settings.constant_watch.to_string()}/>
            </span>
          </div>

          <div>
            <h2>"Conny Settings"</h2>
            <span class="FormRow">
              <p>"Personality Type:"</p>
              <input placeholder={move || user_data.get().conny_settings.personality.to_string()}/>
            </span>
          </div>


        </form>

        <button on:click=update_user>{saving_text}</button>
        <a href="/">"Back to Home"</a>
        <button class="ResetButton" on:click=reset_user>"Reset Settings"</button>

      </div>
    }
}
