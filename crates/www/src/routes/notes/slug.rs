use leptos::{
    component, create_render_effect, create_rw_signal, spawn_local, view, IntoView, SignalGet,
    SignalGetUntracked, SignalSet,
};
use leptos_meta::Title;
use leptos_router::use_route;
use markdown::{Constructs, Options, ParseOptions};
use reqwest::get;
use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use yaml_front_matter::YamlFrontMatter;

use crate::utils::hostname;

#[derive(Clone, Deserialize)]
struct Metadata {
    title: String,
    description: String,
    date: String,
    preview_image_url: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = highlightAll, js_namespace = hljs)]
    fn highlight_all();
}

#[component]
pub fn Note() -> impl IntoView {
    let metadata = create_rw_signal::<Option<Metadata>>(None);
    let note_md = create_rw_signal::<Option<String>>(None);

    create_render_effect(move |_| {
        let route = use_route();

        spawn_local(async move {
            // FIXME: Check slugs against list of notes
            if let Some(slug) = route.params().get_untracked().get("slug") {
                if slug.is_empty() {
                    return;
                }

                let Ok(mut url) = hostname() else {
                    note_md.set(Some("Error: Failed to retrieve hostname".to_string()));
                    return;
                };

                url.set_path(&format!("/assets/notes/{slug}.md"));

                match get(url).await {
                    Ok(response) => {
                        if let Ok(text) = response.text().await {
                            if let Ok(document) = YamlFrontMatter::parse::<Metadata>(&text) {
                                metadata.set(Some(document.metadata));
                            }

                            if let Ok(md) = markdown::to_html_with_options(
                                &text,
                                &Options {
                                    parse: ParseOptions {
                                        constructs: Constructs {
                                            frontmatter: true,
                                            gfm_table: true,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            ) {
                                note_md.set(Some(md));
                            }

                            highlight_all();
                        } else {
                            note_md.set(None);
                        }
                    }
                    Err(error) => {
                        note_md.set(Some(format!("Error: {}", error)));
                    }
                }
            }
        });
    });

    view! {
        <Title
            text={move || metadata.get().map(|meta| meta.title).unwrap_or_default()}
            formatter=|text| format!("{text} — Esteban Borai")
        />
        <section id="note-container" class="pb-10">
            <article id="note-header">
                <figure class="py-4">
                    <img src={move || metadata.get().map(|meta| meta.preview_image_url).unwrap_or_default()} />
                </figure>
                <h1 class="text-3xl font-semibold pb-2">{move || metadata.get().map(|meta| meta.title).unwrap_or_default()}</h1>
                <p>{{move || metadata.get().map(|meta| meta.description).unwrap_or_default()}}</p>
                <time>{move || metadata.get().map(|meta| meta.date).unwrap_or_default()}</time>
            </article>
            <div id="note-content" inner_html={move || note_md.get().unwrap_or_default()}></div>
            <div class="group grid grid-cols-[100px,auto] gap-4 border-t border-gray-400 pt-4">
                <figure class="grayscale group-hover:grayscale-0 rounded-full overflow-hidden flex justify-center items-center h-[100px] w-[100px]">
                    <img src="/assets/images/whoami.jpg" alt="whoami" height="100" width="100" />
                </figure>
                <article>
                    <h1 class="text-lg">"Esteban Borai"</h1>
                    <div class="text-sm space-y-2">
                        <p>
                            "Hi there! I'm a "<u>"Rust Software Engineer"</u>" with 8 years of experience in Systems and Web Programming using Rust & TypeScript."
                        </p>
                        <p>
                            "I'm passionate about Open-Source and enjoy reading books, working out &amp; playing videogames."
                        </p>
                        <p>"I've had the opportunity to work with companies like InfinyOn, GOintegro &amp; Teleperformance."</p>
                    </div>
                </article>
            </div>
        </section>
    }
}
