use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::article::Article;
use crate::components::header::Header;
use crate::components::home::Home;

use super::router::AppRoute;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
            <Header />
            <main>
                <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html!{<Home />},
                            AppRoute::Articles => html! {<h1>{"Articles"}</h1>},
                            AppRoute::Article(id) => html!{<Article id=id />},
                        }
                    })
                />
            </main>
            <footer id="footer">
                <small>
                    {"Made with Rust and 🧉 by Esteban Borai checkout the code "}
                    <a class="link" href="https://github.com/EstebanBorai/estebanborai.github.io" target="_blank">
                        {"here"}
                    </a>
                </small>
            </footer>
          </>
        }
    }
}
