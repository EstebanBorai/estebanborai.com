use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};

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

pub fn article_url_from_location(id: String) -> String {
    format!("http://0.0.0.0:7878/{}.md", id)
}
