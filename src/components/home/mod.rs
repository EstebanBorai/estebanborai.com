mod social_link;

use yew::prelude::*;

use crate::components::icon::{GitHub, LinkedIn, Twitter};

use self::social_link::SocialLink;

pub struct Home {}

impl Component for Home {
    type Message = ();
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
            <section id="home" class="section-wrapper">
                <div class="image-wrapper">
                    <figure class="home-avatar-image">
                        <img
                            src="https://avatars.githubusercontent.com/u/34756077?v=4"
                            height="320"
                            width="320"
                        />
                    </figure>
                </div>
                <article class="personal-resume">
                    <span class="greeting">{"Hi, there! 👋"}</span>
                    <p class="brief">
                        {"I'm a Software Engineer interested in Web Development and Systems Programming"}
                    </p>
                    <ul class="social">
                        <SocialLink href="https://github.com/EstebanBorai">
                            <GitHub />
                        </SocialLink>
                        <SocialLink href="https://www.linkedin.com/in/esteban-b-241ba0135/">
                            <LinkedIn />
                        </SocialLink>
                        <SocialLink href="https://twitter.com/EstebanBorai">
                            <Twitter />
                        </SocialLink>
                    </ul>
                </article>
            </section>
        }
    }
}
