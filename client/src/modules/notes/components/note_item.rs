use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NoteItemProps {
    pub title: String,
    pub description: String,
    pub slug: String,
    pub categories: Vec<String>,
    pub preview_image_url: String,
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
        let background_image =
            format!("background-image: url('{}');", self.props.preview_image_url);

        html! {
            <li class="note-item">
                <a href=link>
                    <figure style=background_image.clone()>{" "}</figure>
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
