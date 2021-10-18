use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NoteItemProps {
    pub title: String,
    pub description: String,
    pub slug: String,
    pub categories: Vec<String>,
}

pub struct NoteItem {
    props: NoteItemProps,
}

impl NoteItem {
    pub fn render_categories(&self) -> Html {
        if self.props.categories.len() >= 1 {
            return html! {
                <ul class="note-tags">
                    {
                        for self.props.categories.iter().map(|category| {
                            html! {
                                <li>{category}</li>
                            }
                        })
                    }
                </ul>
            };
        }

        Html::default()
    }
}

impl Component for NoteItem {
    type Message = ();
    type Properties = NoteItemProps;

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
        let link = format!("/notes/{}", self.props.slug);

        html! {
            <li class="note-item">
                <a href=link>
                    <figure style="background-image: url('https://images.unsplash.com/photo-1538903723116-763313165283?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&ixlib=rb-1.2.1&auto=format&fit=crop&w=1400&q=80');">{" "}</figure>
                    <article>
                        <strong>{self.props.title.clone()}</strong>
                        <p>{self.props.description.clone()}</p>
                        {self.render_categories()}
                    </article>
                </a>
            </li>
        }
    }
}
