mod app;
mod chat;
mod not_found;
mod settings;
mod sign_in;
mod sign_up;
mod upcoming;

use app::*;
use leptos::*;
use leptos_router::*;

use crate::chat::Chat;
use crate::not_found::NotFound;
use crate::settings::Settings;
use crate::sign_in::SignIn;
use crate::sign_up::SignUp;
use crate::upcoming::Upcoming;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <Routes>
                    <Route path="/" view=Home/>
                    //
                    <Route path="/SignUp" view=SignUp/>
                    <Route path="/SignIn" view=SignIn/>
                    //
                    <Route path="/Chat" view=Chat/>
                    <Route path="/Settings" view=Settings/>
                    <Route path="/Upcoming" view=Upcoming/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </Router>
        }
    })
}
