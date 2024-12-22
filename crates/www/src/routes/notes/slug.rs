use leptos::{
    component, create_render_effect, create_rw_signal, spawn_local, view, For, IntoView, SignalGet,
    SignalGetUntracked, SignalSet,
};
use leptos_meta::Title;
use leptos_router::use_route;
use reqwest::get;
use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use yaml_front_matter::YamlFrontMatter;

use crate::utils::hostname;

struct Markdown {
    html: String,
    headings: Vec<String>,
}

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
    let headings = create_rw_signal::<Vec<String>>(Vec::default());

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

                                let md = parse_md_into_html(&document.content);
                                note_md.set(Some(md.html));
                                headings.set(md.headings);

                                highlight_all();
                            }

                            // if let Ok(md) = markdown::to_html_with_options(
                            //     &text,
                            //     &Options {
                            //         parse: ParseOptions {
                            //             constructs: Constructs {
                            //                 frontmatter: true,
                            //                 gfm_table: true,
                            //                 ..Default::default()
                            //             },
                            //             ..Default::default()
                            //         },
                            //         ..Default::default()
                            //     },
                            // ) {
                            //     note_md.set(Some(md));
                            // }
                            //
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
                <figure class="relative py-4 h-[200px] overflow-hidden rounded-md">
                    <img class="absolute h-full w-full" src={move || metadata.get().map(|meta| meta.preview_image_url).unwrap_or_default()} />
                </figure>
                <h1 class="text-3xl font-semibold pb-2">{move || metadata.get().map(|meta| meta.title).unwrap_or_default()}</h1>
                <p>{{move || metadata.get().map(|meta| meta.description).unwrap_or_default()}}</p>
                <time>{move || metadata.get().map(|meta| meta.date).unwrap_or_default()}</time>
            </article>
            <div id="note-content" inner_html={move || note_md.get().unwrap_or_default()}></div>
            <aside id="note-toc" class="hidden md:block">
                <h2 class="text-lg font-semibold">Table of Contents</h2>
                <ul class="space-y-2">
                    <For
                        each=move || headings.get()
                        key=|heading| heading.clone()
                        children=move |heading: String| {
                            view! {
                                <li>
                                    <a href={format!("#{}", heading)}>{heading}</a>
                                </li>
                            }
                        }
                    />
                </ul>
            </aside>
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
                            "I'm passionate about Open-Source and enjoy reading books, working out & playing videogames."
                        </p>
                        <p>"I've had the opportunity to work with companies like InfinyOn, GOintegro & Teleperformance."</p>
                    </div>
                </article>
            </div>
        </section>
    }
}

fn parse_md_into_html(md: &str) -> Markdown {
    use pulldown_cmark::{Event, Parser, Tag};

    let mut text: Vec<String> = Vec::new();
    let mut headings: Vec<String> = Vec::new();
    let mut state = String::new();
    let mut is_heading = false;
    let mut is_codeblock = false;

    for event in Parser::new(md) {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => text.push(r#"<p>"#.to_string()),
                    Tag::Heading { level, .. } => {
                        state.push_str(&format!("<{}>", level.to_string().to_ascii_lowercase()));
                        is_heading = true;
                    }
                    Tag::CodeBlock(_) => {
                        state.push_str("<pre><code>");
                        is_codeblock = true;
                    }
                    Tag::BlockQuote(_) => {
                        text.push(r#"<blockquote class="bg-yellow-100 p-2">"#.to_string());
                    }
                    _ => {} // Tag::HtmlBlock => todo!(),
                            // Tag::List(_) => todo!(),
                            // Tag::Item => todo!(),
                            // Tag::FootnoteDefinition(cow_str) => todo!(),
                            // Tag::DefinitionList => todo!(),
                            // Tag::DefinitionListTitle => todo!(),
                            // Tag::DefinitionListDefinition => todo!(),
                            // Tag::Table(vec) => todo!(),
                            // Tag::TableHead => todo!(),
                            // Tag::TableRow => todo!(),
                            // Tag::TableCell => todo!(),
                            // Tag::Emphasis => todo!(),
                            // Tag::Strong => todo!(),
                            // Tag::Strikethrough => todo!(),
                            // Tag::Link { link_type, dest_url, title, id } => todo!(),
                            // Tag::Image { link_type, dest_url, title, id } => todo!(),
                            // Tag::MetadataBlock(metadata_block_kind) => todo!(),
                }
            }
            Event::End(tag_end) => {
                match tag_end {
                    pulldown_cmark::TagEnd::Paragraph => text.push("</p>".to_string()),
                    pulldown_cmark::TagEnd::CodeBlock => {
                        state.push_str("</code></pre>");
                        is_codeblock = false;
                    }
                    pulldown_cmark::TagEnd::Heading(level) => {
                        state.push_str(&format!("</{}>", level.to_string().to_ascii_lowercase()));
                        is_heading = false;
                    }
                    pulldown_cmark::TagEnd::BlockQuote(_) => {
                        text.push("</blockquote>".to_string());
                    }
                    _ => {} // pulldown_cmark::TagEnd::HtmlBlock => todo!(),
                            // pulldown_cmark::TagEnd::List(_) => todo!(),
                            // pulldown_cmark::TagEnd::Item => todo!(),
                            // pulldown_cmark::TagEnd::FootnoteDefinition => todo!(),
                            // pulldown_cmark::TagEnd::DefinitionList => todo!(),
                            // pulldown_cmark::TagEnd::DefinitionListTitle => todo!(),
                            // pulldown_cmark::TagEnd::DefinitionListDefinition => todo!(),
                            // pulldown_cmark::TagEnd::Table => todo!(),
                            // pulldown_cmark::TagEnd::TableHead => todo!(),
                            // pulldown_cmark::TagEnd::TableRow => todo!(),
                            // pulldown_cmark::TagEnd::TableCell => todo!(),
                            // pulldown_cmark::TagEnd::Emphasis => todo!(),
                            // pulldown_cmark::TagEnd::Strong => todo!(),
                            // pulldown_cmark::TagEnd::Strikethrough => todo!(),
                            // pulldown_cmark::TagEnd::Link => todo!(),
                            // pulldown_cmark::TagEnd::Image => todo!(),
                            // pulldown_cmark::TagEnd::MetadataBlock(metadata_block_kind) => todo!(),
                }

                text.push(state.clone());
                state.clear();
            }
            Event::Text(cow_str) => {
                if !cow_str.is_empty() {
                    if is_heading {
                        headings.push(cow_str.to_string());
                    }

                    if is_codeblock {
                        state.push_str(
                            &cow_str
                                .to_string()
                                .replace("&", "&amp;")
                                .replace("<", "&lt;")
                                .replace(">", "&gt;")
                                .replace("\"", "&quot;")
                                .replace("'", "&#039;"),
                        );
                    } else {
                        state.push_str(&cow_str);
                    }
                }
            }
            Event::Code(cow_str) => {
                state.push_str(&format!("<code>{}</code>", cow_str));
            }
            _ => {} // Event::InlineMath(cow_str) => todo!(),
                    // Event::DisplayMath(cow_str) => todo!(),
                    // Event::Html(cow_str) => todo!(),
                    // Event::InlineHtml(cow_str) => todo!(),
                    // Event::FootnoteReference(cow_str) => todo!(),
                    // Event::SoftBreak => todo!(),
                    // Event::HardBreak => todo!(),
                    // Event::Rule => todo!(),
                    // Event::TaskListMarker(_) => todo!(),
        }
    }

    Markdown {
        html: text.join(""),
        headings,
    }
}
