mod app;
mod backups;
mod chat;
mod helpers;
mod not_found;
mod settings;
mod upcoming;
mod updates;

use app::*;
use leptos::*;
use leptos_router::*;

use crate::backups::Backups;
use crate::chat::Chat;
use crate::not_found::NotFound;
use crate::settings::Settings;
use crate::upcoming::Upcoming;
use crate::updates::Updates;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <Routes>
                    <Route path="/" view=Home ssr=SsrMode::Async/>
                    <Route path="/Chat" view=Chat/>
                    <Route path="/Settings" view=Settings/>
                    <Route path="/Upcoming" view=Upcoming/>
                    <Route path="/Backups" view=Backups/>
                    <Route path="/Updates" view=Updates/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </Router>
        }
    })
}
