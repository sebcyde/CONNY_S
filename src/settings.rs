use leptos::{leptos_dom::logging::console_log, *};
use wasm_bindgen::prelude::*;

use crate::helpers::{
    self,
    functions::{
        get_default_user_config, get_user_details, update_user_details, AppSettings, ConnyConfig,
        SortSettings, UserConfig, UserData,
    },
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Settings() -> impl IntoView {
    let (is_resetting, set_is_resetting) = create_signal(false);
    let (is_saving, set_is_saving) = create_signal(0);

    // Input state

    let (user_data, set_user_data) = create_signal(get_default_user_config());

    let (name, set_name) = create_signal(user_data.get().user_data.user_name);

    let (run_on_startup, set_run_on_startup) =
        create_signal(user_data.get().app_settings.run_on_startup);

    let (persistent_sort, set_persistent_sort) =
        create_signal(user_data.get().app_settings.constant_watch);

    let (personality, set_personality) = create_signal(user_data.get().conny_settings.personality);

    // Functions

    create_effect(move |_| {
        spawn_local(async move {
            let config: UserConfig = helpers::functions::get_user_details().await;
            let content: String = format!("{:?}", config.ser().unwrap());
            console_log(&content);
            set_user_data.set(config);
        });
    });

    let resetting_text = move || match is_resetting.get() {
        true => "Resetting...",
        false => "Reset Settings",
    };

    let saving_text = move || match is_saving.get() {
        0 => "Save",
        1 => "Saving...",
        _ => "Save Succesful",
    };

    let reset_user = move |_| {
        spawn_local(async move {
            set_is_resetting.set(true);
            helpers::functions::reset_user_details().await;
            set_is_resetting.set(false);
        })
    };

    let update_user = move |_| {
        spawn_local(async move {
            set_is_saving.set(1);
            let new_config: UserConfig = UserConfig {
                app_settings: AppSettings {
                    constant_watch: (move || persistent_sort.get_untracked())(),
                    run_on_startup: (move || run_on_startup.get_untracked())(),
                },
                conny_settings: ConnyConfig {
                    personality: (move || personality.get_untracked())(),
                },
                user_data: UserData {
                    user_name: (move || name.get_untracked())(),
                },
                sort_settings: SortSettings {
                    electric_shuffle_uk_location: user_data
                        .get()
                        .sort_settings
                        .electric_shuffle_uk_location,
                    flight_club_uk_location: user_data.get().sort_settings.flight_club_uk_location,
                    red_engine_uk_location: user_data.get().sort_settings.red_engine_uk_location,
                },
            };

            update_user_details(new_config).await;
            let config: UserConfig = get_user_details().await;
            let content: String = format!("{:?}", config.ser().unwrap());
            console_log(&content);

            set_user_data.set(config);
            set_is_saving.set(3);
        });
    };

    view! {
      <div class="settings_page">
        <form>
          <div>
            <h2>"User Settings"</h2>
            <span class="FormRow">
              <p>"User Name:"</p>
              <input
                on:input=move |ev| { set_name.set(event_target_value(&ev)); }
                prop:value=move || name.get()
              />
            </span>
          </div>

          <div>
            <h2>"App Settings"</h2>
            <span class="FormRow">
              <p>"Run On Startup:"</p>
              <input
              type="checkbox"
                on:change=move |ev| { set_run_on_startup.set(event_target_checked(&ev)); }
                prop:value=move || run_on_startup.get()
              />
            </span>

            // <span class="FormRow">
            //   <p>"Sort Files By Date:"</p>
            //   <input type="checkbox" checked={move || user_data.get().app_settings.run_on_startup.to_string()}/>
            // </span>

            <span class="FormRow">
              <p>"Constant File Sort:"</p>
              <input
                type="checkbox"
                on:change=move |ev| { set_persistent_sort.set(event_target_checked(&ev)); }
                prop:value=move || persistent_sort.get()
              />
            </span>
          </div>

          <div>
            <h2>"Conny Settings"</h2>
            <span class="FormRow">
              <p>"Personality Type:"</p>
              <input
                on:input=move |ev| { set_personality.set(event_target_value(&ev)); }
                prop:value=move || personality.get()
              />
            </span>
          </div>


        </form>

        <button on:click=update_user>{saving_text}</button>
        <a href="/">"Back to Home"</a>
        <button class="ResetButton" on:click=reset_user>{resetting_text}</button>

      </div>
    }
}

// view! {
//   <input
//   type="text"
//   class= "new-todo"
//   autofocus=true
//   placeholder="Add todo"
//   on:keydown= move |event| {
//       if event.key() == "Enter" && !event_target_value(&event).is_empty() {
//           let input_value = event_target_value(&event);
//           let new_todo_item = TodoItem { id: new_todo_id(), content: input_value.clone() };
//           set_new_todo.update(|todo| todo.push(new_todo_item));
//           set_default_value.set("");
//       }}
//       prop:value=default_value
//   />
