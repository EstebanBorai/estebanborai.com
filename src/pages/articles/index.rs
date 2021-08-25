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
                <ul id="article-listing">
                    <li class="article-item">
                        <a href="/articles/1">
                            <figure style="background-image: url('https://images.unsplash.com/photo-1482160549825-59d1b23cb208?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&ixlib=rb-1.2.1&auto=format&fit=crop&w=1650&q=80');">{" "}</figure>
                            <article>
                                <strong>{"Testing"}</strong>
                                <p>
                                    {r#"
                                        This is a sample article I'm using to
                                        give a try on this feature of having my
                                        own Article management system. It's a
                                        WIP that I have published.
                                    "#}
                                </p>
                            </article>
                        </a>
                    </li>
                </ul>
            </section>
        }
    }
}
