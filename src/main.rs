mod app;

use app::*;
use leptos::*;
use leptos_router::*;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
            // Navbar ?
                <main class="container">
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/Settings" view=Settings/>
                        <Route path="/Upcoming" view=Upcoming/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </Router>
        }
    })
}
