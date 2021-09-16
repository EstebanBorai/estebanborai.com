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

pub fn article_url_from_location(_id: String) -> String {
    String::from("https://raw.githubusercontent.com/EstebanBorai/EstebanBorai/main/notes/001-installing-the-rust-programming-language-on-windows.md")
}
