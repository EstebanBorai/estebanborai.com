use yew::prelude::*;

use super::IconProps;

pub struct Times {
    class: Option<String>,
}

impl Component for Times {
    type Message = ();
    type Properties = IconProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { class: props.class }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class=self.class.clone().unwrap_or(String::default()) viewBox="0 0 16 16">
                <path d="M1.293 1.293a1 1 0 0 1 1.414 0L8 6.586l5.293-5.293a1 1 0 1 1 1.414 1.414L9.414 8l5.293 5.293a1 1 0 0 1-1.414 1.414L8 9.414l-5.293 5.293a1 1 0 0 1-1.414-1.414L6.586 8 1.293 2.707a1 1 0 0 1 0-1.414z"/>
            </svg>
        }
    }
}
