use yew::prelude::*;
use yew_router::components::RouterAnchor;

use crate::components::icon::{List, Times};
use crate::router::AppRoute;

pub struct Header {
    is_mobile_nav_open: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    OpenMobileNav,
    CloseMobileNav,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Header {
            is_mobile_nav_open: false,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenMobileNav => self.is_mobile_nav_open = true,
            Msg::CloseMobileNav => self.is_mobile_nav_open = false,
        }

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
                    <nav id="desktop-nav">
                        <ul>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Home>
                                    {"Home"}
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li>
                                <RouterAnchor<AppRoute> route=AppRoute::Notes>
                                    {"Notes"}
                                </RouterAnchor<AppRoute>>
                            </li>
                        </ul>
                    </nav>
                    <button id="mobile-nav-button" onclick=self.link.callback(|_| Msg::OpenMobileNav)>
                        <List class="burger-nav-icon".to_string() />
                    </button>
                    {
                        if self.is_mobile_nav_open {
                            html! {
                                <nav id="mobile-nav">
                                    <header>
                                        <button id="mobile-nav-button" onclick=self.link.callback(|_| Msg::CloseMobileNav)>
                                            <Times class="burger-nav-icon".to_string() />
                                        </button>
                                    </header>
                                    <main>
                                        <ul>
                                            <li onclick=self.link.callback(|_| Msg::CloseMobileNav)>
                                                <RouterAnchor<AppRoute> route=AppRoute::Home>
                                                    {"Home"}
                                                </RouterAnchor<AppRoute>>
                                            </li>
                                            <li onclick=self.link.callback(|_| Msg::CloseMobileNav)>
                                                <RouterAnchor<AppRoute> route=AppRoute::Notes>
                                                    {"Notes"}
                                                </RouterAnchor<AppRoute>>
                                            </li>
                                        </ul>
                                    </main>
                                </nav>
                            }
                        } else {
                            Html::default()
                        }
                    }
                </div>
            </header>
        }
    }
}
