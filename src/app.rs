use yew::prelude::*;

use crate::components::header::Header;
use crate::components::home::Home;

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
          <div class="bg-self-black">
            <Header />
            <main>
                <Home />
            </main>
            <footer class="p-4 w-full text-self-white text-center">
                <small>{"Made with code and 🧉 by Esteban Borai"}</small>
            </footer>
          </div>
        }
    }
}
