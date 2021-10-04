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
            <section id="notes">
                <h1>{"Notes"}</h1>
                <p id="section-description">
                    {r#"
                        Notes on findings, and learnings that I keep for the
                        future and in order to share with who needs them
                    "#}
                </p>
                <ul id="notes-listing">
                    <li class="note-item">
                        <a href="/notes/1">
                            <figure style="background-image: url('https://images.unsplash.com/photo-1538903723116-763313165283?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&ixlib=rb-1.2.1&auto=format&fit=crop&w=1400&q=80');">{" "}</figure>
                            <article>
                                <strong>{"Installing The Rust Programming Language on Windows"}</strong>
                                <p>
                                    {r#"
                                        A tutorial on installing the Rust
                                        Programming Language on Windows.
                                    "#}
                                </p>
                                <ul class="note-tags">
                                    <li>{"Rust"}</li>
                                    <li>{"Tutorial"}</li>
                                    <li>{"Windows"}</li>
                                    <li>{"Install"}</li>
                                </ul>
                            </article>
                        </a>
                    </li>
                </ul>
            </section>
        }
    }
}
