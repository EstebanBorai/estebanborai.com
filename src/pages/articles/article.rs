use yew::format::{Nothing, Text};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;
use yew::virtual_dom::VNode;
use yew::web_sys::Node;

use crate::modules::articles::utils;

#[derive(Properties, Clone, PartialEq)]
pub struct ArticleProps {
    pub id: Option<String>,
}

pub struct Article {
    article_markdown: Option<String>,
    article_html: Option<String>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
    props: ArticleProps,
}

pub enum Msg {
    Fetch(String),
    FetchSuccess(String),
    FetchError(String),
    FetchLoading,
}

impl Article {
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

    fn render_article(&self) -> Html {
        html! { <p>{"This is your article"}</p> }
    }
}

impl Component for Article {
    type Message = Msg;
    type Properties = ArticleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Article {
            article_markdown: None,
            article_html: None,
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
                        .callback(|response: Response<Text>| match response.into_body() {
                            Ok(data) => Msg::FetchSuccess(data),
                            Err(err) => Msg::FetchError(err.to_string()),
                        });

                match FetchService::fetch(request, callback) {
                    Ok(fetch_task) => {
                        self.fetch_task = Some(fetch_task);

                        Msg::FetchLoading
                    }
                    Err(err) => Msg::FetchError(err.to_string()),
                };

                true
            }
            Msg::FetchError(err) => {
                self.error = Some(err);

                true
            }
            Msg::FetchLoading => true,
            Msg::FetchSuccess(markdown) => {
                self.article_markdown = Some(markdown.clone());
                self.article_html = Some(utils::parse_markdown(markdown.as_str()));

                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(id) = self.props.id.clone() {
                let article_url = utils::article_url_from_location(id);

                self.update(Msg::Fetch(article_url));
            }
        }
    }

    fn view(&self) -> Html {
        if let Some(parsed_html) = self.article_html.clone() {
            let wrapper = yew::web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();

            wrapper.set_inner_html(&parsed_html);

            let node = Node::from(wrapper);
            let vnode = VNode::VRef(node);

            return html! {
                <section id="article">
                    <div id="article-container">
                        {vnode}
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
