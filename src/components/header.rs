use yew::prelude::*;
use yew_router::components::RouterAnchor;

use crate::router::AppRoute;

pub struct Header {}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header id="header">
                <div class="wrapper">
                    <h1>{"Esteban Borai"}</h1>
                    <nav>
                        <ul>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Home classes="router,active">
                                    {"Home"}
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Articles>
                                    {"Articles"}
                                </RouterAnchor<AppRoute>>
                            </li>
                        </ul>
                    </nav>
                </div>
            </header>
        }
    }
}
