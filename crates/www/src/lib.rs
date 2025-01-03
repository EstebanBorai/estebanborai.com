mod routes;
mod utils;

use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Routes, A};

use self::routes::about::About;
use self::routes::bookshelf::Bookshelf;
use self::routes::home::Home;
use self::routes::notes::slug::Note;
use self::routes::notes::Notes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <header class="flex items-center justify-between py-2 w-11/12 md:w-[1100px] mx-auto py-6">
            <h1 class="font-semibold text-xl">
                <a href="/">Esteban Borai</a>
            </h1>
            <nav class="text-sm">
                <ul class="flex items-center space-x-4">
                    <li>
                        <A href="/">"Home"</A>
                    </li>
                    <li>
                        <A href="/notes">"Notes"</A>
                    </li>
                    <li>
                        <A href="/bookshelf">"Bookshelf"</A>
                    </li>
                    <li>
                        <A href="/about">"About"</A>
                    </li>
                </ul>
            </nav>
        </header>
        <main class="min-h-screen w-11/12 md:w-[1100px] mx-auto">
            <Routes>
                <Route path="/" view=Home />
                <Route path="/notes" view=Notes />
                <Route path="/notes/:slug" view=Note />
                <Route path="/bookshelf" view=Bookshelf />
                <Route path="/about" view=About />
            </Routes>
        </main>
        <footer class="flex items-center justify-center p-4">
            <small>"Made with 🧉 and ❤️ by Esteban Borai © 2017 - 2025"</small>
        </footer>
    }
}
