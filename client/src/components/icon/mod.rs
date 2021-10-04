mod github;
mod linkedin;
mod list;
mod times;
mod twitter;

pub use github::GitHub;
pub use linkedin::LinkedIn;
pub use list::List;
pub use times::Times;
pub use twitter::Twitter;

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    #[prop_or(None)]
    pub class: Option<String>,
}
