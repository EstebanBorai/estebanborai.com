mod routes;
mod utils;

use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

use self::routes::home::Home;
use self::routes::notes::slug::Note;
use self::routes::notes::Notes;

// const POST: &str = include_str!("../assets/notes/deploy-rust-to-fly-io.md");

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home />
                <Route path="/notes" view=Notes />
                <Route path="/notes/:slug" view=Note />
            </Routes>
        </Router>
    }
}
