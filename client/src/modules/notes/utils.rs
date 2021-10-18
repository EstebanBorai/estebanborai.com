use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};

pub fn parse_markdown(markdown: &str) -> String {
    let mut opts = Options::empty();

    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, opts);
    let mut parsed = String::default();

    push_html(&mut parsed, parser);
    parsed
}
