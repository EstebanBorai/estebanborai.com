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
        <header class="flex items-center justify-between px-4 py-2">
            <h1 class="font-semibold">
                <a href="/">Esteban Borai</a>
            </h1>
            <nav>
                <ul class="flex items-center space-x-4">
                    <li>
                        <a href="/">Home</a>
                    </li>
                    <li>
                        <a href="/notes">Notes</a>
                    </li>
                </ul>
            </nav>
        </header>
        <main class="min-h-screen w-11/12 md:w-[1100px] mx-auto">
            <Router>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/notes" view=Notes />
                    <Route path="/notes/:slug" view=Note />
                </Routes>
            </Router>
        </main>
        <footer class="flex items-center justify-center p-4">
            <small>"Made with 🧉 and ❤️ by Esteban Borai © 2017 - 2024"</small>
        </footer>
    }
}
