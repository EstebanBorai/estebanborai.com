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
    String::from("https://gist.githubusercontent.com/EstebanBorai/cdc9afd6097f70b801a0d2f9fbee2d03/raw/65e5ad53e3c28b17140b93d3ca0aa8bd1a9d05b0/testing_markdown_syntax.md")
}
