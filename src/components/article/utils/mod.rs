use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};
use web_sys::Url;
use yew::web_sys::window;

pub fn parse_markdown(value: &str) -> String {
    let mut opts = Options::empty();

    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&value, opts);
    let mut parsed = String::default();

    push_html(&mut parsed, parser);

    parsed
}

pub fn location() -> Option<String> {
    if let Some(win) = window() {
        let loc = win.location();

        if let Ok(href) = loc.href() {
            return Some(href);
        }

        return None;
    }

    None
}

pub fn article_url_from_location(location: String) -> Option<String> {
    if let Ok(url) = Url::new(location.as_str()) {
        let url_search_params = url.search_params();
        if let Some(art_key) = url_search_params.get("art") {
            return Some(format!("http://0.0.0.0:7878/{}", art_key));
        }

        return None;
    }

    None
}
