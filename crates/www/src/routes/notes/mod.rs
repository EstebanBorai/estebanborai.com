pub mod slug;

use leptos::{
    component, create_render_effect, create_rw_signal, spawn_local, view, For, IntoView, SignalGet,
    SignalSet,
};
use leptos_meta::Title;
use proto::{NotesIndex, RichNoteMetadata};
use reqwest::get;

use crate::utils::hostname;

#[component]
pub fn Notes() -> impl IntoView {
    let notes_index = create_rw_signal::<Vec<RichNoteMetadata>>(Vec::default());

    create_render_effect(move |_| {
        spawn_local(async move {
            let Ok(mut url) = hostname() else {
                return;
            };

            url.set_path("/assets/notes_index.json");

            match get(url).await {
                Ok(response) => {
                    if let Ok(idx) = response.json::<NotesIndex>().await {
                        notes_index.set(idx.0.clone());
                    }
                }
                Err(error) => {
                    leptos::logging::error!("Failed to fetch notes {error}");
                }
            }
        });
    });

    view! {
        <Title
            text={move || "Notes"}
            formatter=|text| format!("{text} — Esteban Borai")
        />
        <div>
            <h2 class="text-lg py-2">Notes</h2>
            <ul class="py-2">
                <For
                  each=move || notes_index.get()
                  key=|note| note.slug.clone()
                  children=move |RichNoteMetadata {
                      meta,
                      slug,
                  }| {
                    view! {
                        <article class="self-start flex flex-col justify-start w-full col-span-4 border-b mb-4">
                            <h3 class="font-semibold hover:underline">
                                <a href={format!("/notes/{slug}")}>
                                    {meta.title}
                                </a>
                            </h3>
                            <p aria-label="Walkthrough on how to deploy a Rust application to Fly.io" class="py-2 text-sm truncate">
                            {meta.description}
                            </p>
                            <ul class="flex justify-start items-start flex-wrap gap-2 py-2">
                                <For
                                    each=move || meta.categories.clone()
                                    key=|cat| cat.clone()
                                    children=move |cat: String| {
                                        view! {
                                            <span class="uppercase inline-block bg-lt-alte dark:bg-dk-alte rounded-md text-center">
                                                <span class="text-xs bg-gray-100 px-2 py-0.25 rounded-md">{cat}</span>
                                            </span>
                                        }
                                    }
                                />
                            </ul>
                            <div class="flex">
                                <span class="flex items-center">
                                    <time class="py-2 text-xs uppercase" datetime={meta.date.to_string()}>{meta.date.to_string()}</time>
                                </span>
                            </div>
                        </article>
                    }
                  }
                />
            </ul>
        </div>
    }
}
