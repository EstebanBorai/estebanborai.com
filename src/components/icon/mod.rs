mod github;
mod linkedin;
mod twitter;

pub use github::GitHub;
pub use linkedin::LinkedIn;
pub use twitter::Twitter;

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    #[prop_or(None)]
    pub class: Option<String>,
}
