use leptos::{component, view, IntoView};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="space-y-2">
            <p>
                "Hi! I'm Esteban Borai a Rust Software Developer who enjoys writing Open-Source Software, Web Development and Systems Programming."
            </p>
        </div>
    }
}
