use chrono::Datelike;
use leptos::{component, view, IntoView};

const CAREER_START_YEAR: i32 = 2017;
const RUST_START_YEAR: i32 = 2019;

#[component]
pub fn About() -> impl IntoView {
    let year = chrono::Utc::now().year();

    view! {
        <div class="space-y-2">
            <h2 class="font-semibold">"Software Development Career"</h2>
            <p>
                "I have been writing Rust for the last "{year - RUST_START_YEAR}" years, before that I was focused on Web Development "
                "using TypeScript, I mastered frameworks like React, EmberJS and Svelte working on different Software Industry "
                "fields including E-Commerce, Social Networks and Real-Time Communications."
            </p>
            <p>
                "I started my career as Web Developer back in "{CAREER_START_YEAR}" ("{year - CAREER_START_YEAR}" years ago)."
            </p>
        </div>
    }
}
