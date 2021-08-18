mod header;

use yew::prelude::*;

use self::header::Header;

#[derive(Properties, Clone)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Layout {
    props: LayoutProps,
}

impl Component for Layout {
    type Message = ();
    type Properties = LayoutProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Layout { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="layout">
              <Header />
              <main id="page-container">
                { self.props.children.clone() }
              </main>
              <footer id="footer">
                  <small>
                      {"Made with Rust and 🧉 by Esteban Borai checkout the code "}
                      <a class="link" href="https://github.com/EstebanBorai/estebanborai.github.io" target="_blank">
                          {"here"}
                      </a>
                  </small>
              </footer>
            </div>
        }
    }
}
