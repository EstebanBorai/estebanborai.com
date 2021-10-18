use anyhow::Error;
use domain::{NoteContent, NoteMetadata};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::{ConsoleService, FetchService};
use yew::virtual_dom::VNode;
use yew::web_sys::Node;

use crate::endpoint;
use crate::modules::notes::utils::parse_markdown;

#[derive(Properties, Clone, PartialEq)]
pub struct NoteProps {
    pub slug: Option<String>,
}

pub struct Note {
    metadata: Option<NoteMetadata>,
    html: Option<String>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
    props: NoteProps,
}

pub enum Msg {
    Fetch(String),
    FetchSuccess(NoteContent),
    FetchError(String),
    FetchLoading,
}

impl Note {
    fn render_loading(&self) -> Html {
        if self.fetch_task.is_some() {
            html! {
                <p>{ "Fetching data..." }</p>
            }
        } else {
            html! { <p></p> }
        }
    }

    fn render_error(&self, error: String) -> Html {
        html! { <p>{ error.clone() }</p> }
    }
}

impl Component for Note {
    type Message = Msg;
    type Properties = NoteProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Note {
            metadata: None,
            html: None,
            fetch_task: None,
            link,
            error: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Fetch(url) => {
                let request = Request::get(url)
                    .body(Nothing)
                    .expect("Failed to fetch article");

                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<NoteContent, Error>>>| {
                            let Json(note_content) = response.into_body();

                            match note_content {
                                Ok(note_content) => Msg::FetchSuccess(note_content),
                                Err(err) => {
                                    ConsoleService::error(&err.to_string());
                                    Msg::FetchError(String::from(
                                        "Failed to fetch notes from server",
                                    ))
                                }
                            }
                        });

                match FetchService::fetch(request, callback) {
                    Ok(fetch_task) => {
                        self.fetch_task = Some(fetch_task);

                        Msg::FetchLoading
                    }
                    Err(err) => {
                        let error_string = err.to_string();

                        ConsoleService::error(&error_string);
                        Msg::FetchError(error_string)
                    }
                };

                true
            }
            Msg::FetchError(err) => {
                self.error = Some(err);

                true
            }
            Msg::FetchLoading => true,
            Msg::FetchSuccess(note_content) => {
                let html = parse_markdown(note_content.content.as_str());

                self.metadata = Some(note_content.metadata.clone());
                self.html = Some(html);

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(slug) = self.props.slug.clone() {
                self.update(Msg::Fetch(endpoint!(&format!("/api/v1/notes/{}", slug))));
            }
        }
    }

    fn view(&self) -> Html {
        if let Some(html) = self.html.clone() {
            let metadata = self.metadata.clone().unwrap();
            let wrapper = yew::web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();

            wrapper.set_inner_html(&html);

            let node = Node::from(wrapper);
            let vnode = VNode::VRef(node);

            return html! {
                <section id="notes">
                    <div id="notes-container">
                        <header>
                            <h1 id="note-title">{metadata.title}</h1>
                        </header>
                        <main>
                            {vnode}
                        </main>
                    </div>
                </section>
            };
        }

        if let Some(error) = self.error.clone() {
            return self.render_error(error);
        }

        self.render_loading()
    }
}
