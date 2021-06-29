use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SocialLinkProps {
    pub href: String,
    pub children: Children,
}

pub struct SocialLink {
    props: SocialLinkProps,
}

impl Component for SocialLink {
    type Message = ();
    type Properties = SocialLinkProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props: props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <li class="social-link">
                <a href=self.props.href.clone() target="_blank">
                    {self.props.children.clone()}
                </a>
            </li>
        }
    }
}
