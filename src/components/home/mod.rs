use yew::prelude::*;

pub struct Home {}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="flex items-center section-height justify-center relative">
                <div class="flex items-center justify-center w-3/6 ml-auto">
                    <div class="inline-block mr-3 ml-auto">
                        <figure class="home-avatar-image rounded-full overflow-hidden">
                            <img src="https://avatars.githubusercontent.com/u/34756077?v=4" height="320" width="320" />
                        </figure>
                    </div>
                </div>
                <div class="flex items-center justify-center w-3/6 mr-auto">
                    <p class="font-display text-4xl text-self-white text-left w-full">
                        {"Hi! I'm Esteban Borai"}
                    </p>
                </div>
            </section>
        }
    }
}
