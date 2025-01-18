use leptos::{component, view, IntoView, TextProp};

#[component]
pub fn Section(
    #[prop(into, optional)] title: TextProp,
    #[prop(into, optional)] description: TextProp,
) -> impl IntoView {
    view! {
        <div id="section-title pb-4">
            <h2 class="text-lg pb-1">{title}</h2>
            <p class="text-sm text-gray-600">{description}</p>
        </div>
    }
}
