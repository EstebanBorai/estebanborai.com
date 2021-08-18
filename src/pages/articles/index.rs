use yew::prelude::*;

pub struct Index {}

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Index {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="articles">
                <h1>{"Articles"}</h1>
                <p>{"Notes and articles I'm planning to write at some point in time will live here."}</p>
                <div>
                    {"Opps! This looks like empty, theres no articles available yet. Come back tomorrow!"}
                </div>
            </section>
        }
    }
}
