use chrono::Datelike;
use leptos::{component, view, IntoView};

const CAREER_START_YEAR: i32 = 2017;
const RUST_START_YEAR: i32 = 2019;

#[component]
pub fn Home() -> impl IntoView {
    let year = chrono::Utc::now().year();

    view! {
        <div class="space-y-2">
            <section class="text-gray-600">
                <article class="text-sm">
                    <p>"Software Developer"</p>
                    <p>"South America (UTC-3)"</p>
                </article>
            </section>
            <section>
                <h3 class="font-semibold pb-2 text-lg text-gray-800">"Bio"</h3>
                <article class="text-sm text-gray-600">
                    <p>
                        "I have been writing Rust for the last "{year - RUST_START_YEAR}" years, before that I was focused on Web Development "
                        "using TypeScript, I mastered frameworks like React, EmberJS and Svelte working on different Software Industry "
                        "fields including E-Commerce, Social Networks and Real-Time Communications."
                    </p>
                    <p>
                        "I started my career as Web Developer back in "{CAREER_START_YEAR}" ("{year - CAREER_START_YEAR}" years ago)."
                    </p>
                </article>
            </section>
        </div>
    }
}
