use leptos::{
    component, create_render_effect, create_rw_signal, spawn_local, view, For, IntoView, SignalGet,
    SignalSet,
};
use leptos_meta::Title;
use reqwest::get;

use proto::BookshelfBook;

use crate::utils::hostname;

#[component]
pub fn Bookshelf() -> impl IntoView {
    let bookshelf = create_rw_signal::<Vec<BookshelfBook>>(Vec::default());

    create_render_effect(move |_| {
        spawn_local(async move {
            let Ok(mut url) = hostname() else {
                return;
            };

            url.set_path("/assets/db/bookshelf.json");

            match get(url).await {
                Ok(response) => {
                    if let Ok(data) = response.json::<Vec<BookshelfBook>>().await {
                        bookshelf.set(data);
                    }
                }
                Err(error) => {
                    leptos::logging::error!("Failed to fetch bookshelf {error}");
                }
            }
        });
    });

    view! {
        <Title
            text={move || "Bookshelf"}
            formatter=|text| format!("{text} — Esteban Borai")
        />
        <div>
            <h2 class="text-lg">"Bookshelf"</h2>
            <p class="text-sm text-gray-600">"Books I have read and recommend reading for other Software Developers in different fields and levels of expertise."</p>
            <ul class="py-4">
                <For
                  each=move || bookshelf.get()
                  key=|book| book.isbn_10.clone()
                  children=move |BookshelfBook {
                      title,
                      author,
                      isbn_10,
                      isbn_13,
                      publisher,
                      read_on,
                      review,
                      shopping,
                  }| {
                    view! {
                        <article class="self-start flex flex-col justify-start w-full col-span-4 border-b mb-4">
                            <h3 class="font-semibold">
                                {title}
                            </h3>
                            <p class="py-2 text-sm">
                                {review}
                            </p>
                            <dl class="flex flex-col space-y-2 py-2 text-sm md:space-y-0 md:grid gap-1 md:grid-cols-[repeat(6,16.66%)]">
                                <dt class="font-semibold row-start-1 col-start-1 col-span-1">"Author"</dt>
                                <dd class="row-start-2 col-start-1 col-span-1">{author}</dd>
                                <dt class="font-semibold row-start-1 col-start-2 col-span-1">"ISBN-10"</dt>
                                <dd class="row-start-2 col-start-2 col-span-1 font-mono">{isbn_10}</dd>
                                <dt class="font-semibold row-start-1 col-start-3 col-span-1">"ISBN-13"</dt>
                                <dd class="row-start-2 col-start-3 col-span-1 font-mono">{isbn_13}</dd>
                                <dt class="font-semibold row-start-1 col-start-4 col-span-1">"Publisher"</dt>
                                <dd class="row-start-2 col-start-4 col-span-1">{publisher}</dd>
                                <dt class="font-semibold col-start-5 col-span-1">"Read On"</dt>
                                <dd class="row-start-2 col-start-5 col-span-1">{read_on}</dd>
                                <dt class="font-semibold col-start-6 col-span-1">"Purchase"</dt>
                                <dd>
                                    <a class="text-blue-400 hover:text-blue-600 underline" href={shopping.amazon} target="_blank">"Amazon"</a>
                                </dd>
                            </dl>
                        </article>
                    }
                  }
                />
            </ul>
        </div>
    }
}
