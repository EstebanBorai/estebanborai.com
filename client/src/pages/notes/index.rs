use anyhow::Error;
use domain::NoteMetadata;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};

use crate::endpoint;
use crate::modules::notes::components::note_item::NoteItem;

pub struct Index {
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    link: ComponentLink<Self>,
    notes: Option<Vec<NoteMetadata>>,
}

#[derive(Debug)]
pub enum Msg {
    Fetch,
    FetchSuccess(Vec<NoteMetadata>),
    FetchError(String),
}

impl Index {
    fn render_body(&self) -> Html {
        if let Some(error) = &self.error {
            return html! {
                <div>
                    {error.clone()}
                </div>
            };
        }

        if let Some(notes) = &self.notes {
            return html! {
                <ul id="notes-listing">
                    {
                        for notes.iter().map(|note| {
                            html! {
                                <NoteItem
                                    title=note.title.clone()
                                    description=note.description.clone()
                                    slug=note.slug.clone()
                                    categories=note.categories.clone()
                                />
                            }
                        })
                    }
                </ul>
            };
        }

        html! {
            <div>
                {"Loading..."}
            </div>
        }
    }
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Index {
            error: None,
            fetch_task: None,
            link,
            notes: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Fetch => {
                let request = Request::get(endpoint!("/api/v1/notes"))
                    .body(Nothing)
                    .expect("Could not build request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<NoteMetadata>, Error>>>| {
                        let Json(data) = response.into_body();

                        match data {
                            Ok(notes) => Msg::FetchSuccess(notes),
                            Err(err) => {
                                ConsoleService::error(&err.to_string());
                                Msg::FetchError(String::from("Failed to fetch notes from server"))
                            }
                        }
                    },
                );

                let task =
                    FetchService::fetch(request, callback).expect("Failed to perform request");

                self.fetch_task = Some(task);

                true
            }
            Msg::FetchSuccess(notes) => {
                self.notes = Some(notes);
                self.fetch_task = None;

                true
            }
            Msg::FetchError(error_message) => {
                self.error = Some(error_message);
                self.fetch_task = None;

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update(Msg::Fetch);
        }
    }

    fn view(&self) -> Html {
        html! {
            <section id="notes">
                <h1>{"Notes"}</h1>
                <p id="section-description">
                    {r#"
                        Notes on findings, and learnings that I keep for the
                        future and in order to share with who needs them
                    "#}
                </p>
                {self.render_body()}
            </section>
        }
    }
}
