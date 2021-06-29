use yew::prelude::*;

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
                </div>
            </header>
        }
    }
}
